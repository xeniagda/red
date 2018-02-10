pub mod parse;

use std::io;
use std::io::{stdout, Write, Read};
use std::option;
use std::fs::File;
use std::process::exit;

use termion::{color, style};
use lazysort::{Sorted, SortedBy};
use regex::Regex;
use regex;

use readline::read_line;
use red_master::RedMaster;
use red_buffer::RedBuffer;
use range::Range;
use config;

static SEL_CHARS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#%&'()*+,-./:<;=@>?[\\]^_`{|}";

#[derive(Debug, Clone)]
pub enum Action {
    Insert,  // Inserts text before a line
    Change,  // Change the content of a line
    Append,  // Append text at the end of a line
    Delete(String),  // Optionally puts into a registers
    Yank(String),
    Paste(String),

    InsertText(String), // Insert text in the beginning of a line
    AppendText(String), // Append text to the end of a line

    Registers(Option<String>), // Show the contets of all or any register

    Clear, // Clear the screen

    CopyTo(Range),   // Copy a range from one place to another
    Substitute(String, String), // Substitute a by b

    Print,   // Print a range with line number
    Print_,   // Print a line

    BufList, // List all buffers
    BufChange(usize), // Change buffer
    BufNew(Option<String>), // New buffer
    BufDel(bool), // Delete buffer (force)

    Write(String),
    Edit(bool, String)
}

#[derive(Debug)]
pub enum ActionErr {
    OutOfBounds,
    NoRange,
    IO(io::Error),
    NoSuchRegisters,
    Regex,
    Other,
}

impl Action {
    pub fn apply(self, master: &mut RedMaster) -> Result<(), ActionErr> {
        let modified = match self {
            Action::Delete(reg) => {
                let removed_lines = {
                    let file = master.curr_buf_mut();

                    let removed_lines: Vec<_> = file.lines.clone().into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| if file.cursor.lines.contains(&i) { Some(line) } else { None })
                        .collect();

                    file.lines = file.clone().lines.into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| if !file.cursor.lines.contains(&i) { Some(line) } else { None })
                        .collect();

                    removed_lines
                };

                let modified = !removed_lines.is_empty();
                master.registers.insert(reg.into(), removed_lines);

                modified
            }
            Action::Insert => {
                let file = master.curr_buf_mut();
                let mut modified = false;
                loop {
                    let to_insert = read_line("i> ");
                    if let Err(_) = to_insert {
                        break;
                    }
                    let to_insert = to_insert.unwrap();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line, to_insert.clone())?;
                    }
                    modified = true;
                }

                modified
            }
            Action::Append => {
                let file = master.curr_buf_mut();
                let mut modified = false;
                let mut i = 1;
                loop {
                    let to_insert = read_line("a> ");
                    if let Err(_) = to_insert {
                        break;
                    }
                    let to_insert = to_insert.unwrap();

                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line + i, to_insert.clone())?;
                    }
                    i += 1;
                    modified = true;
                }
                modified
            }
            Action::Change => {
                let file = master.curr_buf_mut();
                'outer: for line in file.cursor.lines.clone().into_iter().sorted() {
                    let content = file.lines[line].clone();
                    let mut sel_chars = SEL_CHARS.to_string();
                    sel_chars.truncate(content.len());
                    sel_chars += "$";

                    println!("  {}", content);
                    println!("{}  {}{}", color::Fg(color::Cyan), sel_chars, style::Reset);

                    let mut targets;
                    loop {
                        let line = read_line("T> ");
                        if let Err(_) = line {
                            break 'outer;
                        }

                        targets = line.unwrap();

                        if targets.len() == 0 {
                            continue 'outer;
                        }
                        if targets.len() <= 2 {
                            break;
                        }
                    }

                    let start = sel_chars.find(targets.chars().nth(0)?)?;
                    let end = sel_chars.find(targets.chars().last()?)? + 1;

                    let text = read_line("c> ");
                    if let Err(_) = text {
                        break;
                    }
                    let text = text.unwrap();


                    let line_before = file.lines[line].clone();
                    file.lines[line] = line_before[..start].to_string();
                    file.lines[line].push_str(&text);
                    if end < content.len() {
                        file.lines[line].push_str(&line_before[end..]);
                    }
                }

                true
            }
            Action::AppendText(text) => {
                let file = master.curr_buf_mut();

                for line_nr in &file.cursor.lines {
                    if let Some(line) = file.lines.get_mut(*line_nr) {
                        *line = format!("{}{}", line, text);
                    }
                }

                file.cursor.lines.len() > 0 && text != ""
            }
            Action::InsertText(text) => {
                let file = master.curr_buf_mut();

                for line_nr in &file.cursor.lines {
                    if let Some(line) = file.lines.get_mut(*line_nr) {
                        *line = format!("{}{}", text, line);
                    }
                }

                file.cursor.lines.len() > 0 && text != ""
            }
            Action::Yank(reg) => {
                let lines = {
                    let file = master.curr_buf_mut();

                    file.lines.clone().into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| if file.cursor.lines.contains(&i) { Some(line) } else { None })
                        .collect()
                };

                master.registers.insert(reg.into(), lines);

                false
            }
            Action::Paste(reg) => {
                if let Some(lines_to_paste) = master.registers.clone().get(&reg.into()) {
                    let file = master.curr_buf_mut();

                    let mut last_line: Option<usize> = None;

                    let lines_locations: Vec<_> =
                        file.cursor.lines.clone().into_iter()
                        .sorted().collect();

                    for (i, line) in lines_to_paste.into_iter().enumerate() {
                        if let Some(location) = lines_locations.get(i) {
                            file.lines.insert(*location, line.clone());
                            last_line = Some(*location);
                        } else if let Some(location) = last_line {
                            file.lines.insert(location + 1, line.clone());
                            last_line = Some(location + 1);
                        } else {
                            return Err(ActionErr::NoRange);
                        }
                    }

                    true
                } else {
                    return Err(ActionErr::NoSuchRegisters);
                }

            }
            Action::Registers(Some(reg)) => {
                if let Some(content) = master.registers.clone().get(&reg.clone().into()) {
                    println!("{}:", reg);
                    for line in content {
                        println!("    {}", line);
                    }
                } else {
                    return Err(ActionErr::NoSuchRegisters);
                }
                false
            }
            Action::Registers(None) => {
                for reg in master.registers.clone().keys() {
                    Action::Registers(Some((&*reg.clone()).into())).apply(master)?;
                }
                false
            }
            Action::Clear => {
                print!("\x1B[2J\x1B[1;1H");
                stdout().flush().expect("Can't flush STDOUT");
                false
            }
            Action::CopyTo(to) => {
                let file = master.curr_buf_mut();
                let lines_to_yank: Vec<_> =
                        file.cursor.lines.clone().into_iter()
                        .sorted()
                        .map(|l| file.lines[l].clone())
                        .collect();
                let res_lines: Vec<usize> = to.lines.into_iter().sorted().collect();
                let mut last_line: Option<usize> = None;

                for (i, line) in lines_to_yank.into_iter().enumerate() {
                    let res_pos = res_lines.get(i).map(|x| *x).unwrap_or_else(|| last_line.unwrap() + 1);
                    file.insert_line(res_pos, line)?;
                    last_line = Some(res_pos);
                }
                true
            }
            Action::Substitute(pat, rep) => {
                let file = master.curr_buf_mut();
                let replacer: &str = &rep;
                let rpat = Regex::new(&pat)?;
                let mut count = 0;
                let mut lines = 0;
                for i in file.cursor.lines.iter() {
                    let line = file.lines[*i].clone();
                    let matches_on_line = rpat.find_iter(&line).count();
                    count += matches_on_line;
                    lines += if matches_on_line > 0 { 1 } else { 0 };
                    let replaced = rpat.replace_all(&line, replacer).into_owned();

                    file.lines[*i] = replaced;
                }
                if !config::CONF.lock().unwrap().silent {
                    println!("Did {} replacements on {} lines", count, lines);
                }
                count > 0
            }
            Action::BufList => {
                for (i, buf) in master.buffers.iter().enumerate() {
                    if &i == master.curr_buf_idx() {
                        print!("* ");
                    } else {
                        print!("  ");
                    }
                    print!("{}{}: ", color::Fg(color::Cyan), i);
                    match buf.filename {
                        Some(ref name) => {
                            print!("{}{}", color::Fg(color::Green), name);
                        }
                        None => {
                            print!("{}[untitled]", color::Fg(color::Green));
                        }
                    }
                    if !buf.saved {
                        print!(" [+]");
                    }
                    println!("{}", style::Reset);
                }
                false
            }
            Action::BufDel(force) => {
                if !master.curr_buf().saved && !force {
                    eprintln!("Not saved!");
                    return Err(ActionErr::Other);
                }
                if master.buffers.len() == 1 {
                    exit(0);
                }
                if let Some(ref name) = master.curr_buf().filename {
                    println!("Closing {}", name);
                } else {
                    println!("Closing [untitled]");
                }

                let idx = master.curr_buf_idx().clone();
                master.buffers.remove(idx);
                if idx > 0 {
                    master.change_buffer(idx - 1)?;
                }
                false
            }
            Action::BufNew(None) => {
                master.buffers.push(RedBuffer::empty());
                let buffers = master.buffers.len();
                master.change_buffer(buffers - 1)?;

                if !config::CONF.lock().unwrap().silent {
                    println!("Editing new file!");
                }

                false
            }
            Action::BufNew(Some(file_name)) => {
                master.buffers.push(RedBuffer::empty());
                let buffers = master.buffers.len();
                master.change_buffer(buffers - 1)?;

                Action::Edit(true, file_name).apply(master)?;
                false
            }
            Action::BufChange(i) => {
                master.change_buffer(i)?;
                false
            }
            Action::Write(n_path) => {
                let file = master.curr_buf_mut();
                let path =
                    if n_path.trim().is_empty() {
                        file.clone().filename?
                    } else { n_path };
                let mut out = File::create(path.trim())?;
                let mut first = true;
                for line in file.lines.iter() {
                    if !first {
                        out.write(&[10])?;
                    }
                    first = false;
                    out.write(line.bytes().collect::<Vec<u8>>().as_slice())?;
                }
                file.filename = Some(path);
                file.saved = true;

                false
            }
            Action::Edit(force, path) => {
                let file = master.curr_buf_mut();
                if !file.saved && !force {
                    eprintln!("Not saved!");
                    return Err(ActionErr::Other);
                }
                let mut f = File::open(path.trim());
                if f.is_ok() {
                    let mut f = f.unwrap();
                    let mut content = String::new();
                    f.read_to_string(&mut content)?;

                    file.lines = content.lines().map(|x| x.to_string()).collect();
                    file.cursor = Range::empty();
                    if !config::CONF.lock().unwrap().silent {
                        println!("Editing {} [{}]", path.trim(), file.lines.len());
                    }
                } else {
                    file.lines = vec![];
                    file.cursor = Range::empty();
                    if !config::CONF.lock().unwrap().silent {
                        println!("Editing {} [NEW]", path.trim());
                    }
                }
                file.saved = true;
                file.filename = Some(path.trim().to_string());

                false
            }
            Action::Print => {
                let file = master.curr_buf_mut();
                let mut next = None;
                let leading_digits = file.cursor.lines.clone().into_iter()
                        .map(|x| ((x + 1) as f32).log10().floor() as usize)
                        .max()
                        .unwrap_or(0) + 1;

                for line in file.cursor.lines.clone().into_iter().sorted() {
                    if next.is_some() && Some(line) != next {
                        println!("{}    ...", color::Fg(color::Green));
                    }
                    match file.lines.get(line) {
                        Some(content) => {
                            print!("{3}{1:0$}{2} ", leading_digits, line, style::Reset, color::Fg(color::Cyan));
                            println!("{}", content);
                        }
                        None => {
                            println!("{3}{1:0$}{2}", leading_digits, line, style::Reset, color::Fg(color::Red));
                        }
                    }
                    next = Some(line + 1);
                }
                false
            }
            Action::Print_ => {
                let file = master.curr_buf_mut();
                for line in file.cursor.lines.iter().sorted() {
                    println!("{}", file.lines[*line]);
                }
                false
            }
        };

        if modified {
            master.curr_buf_mut().saved = false;
        }
        Ok(())
    }
}

impl From<io::Error> for ActionErr {
    fn from(err: io::Error) -> ActionErr {
        ActionErr::IO(err)
    }
}

impl From<option::NoneError> for ActionErr {
    fn from(_: option::NoneError) -> ActionErr {
        ActionErr::OutOfBounds
    }
}

impl From<regex::Error> for ActionErr {
    fn from(_: regex::Error) -> ActionErr {
        ActionErr::Regex
    }
}