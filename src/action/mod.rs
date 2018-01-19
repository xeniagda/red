pub mod parse;

use std::io::{stdin, stdout, Write, Read};
use std::io;
use std::option;
use std::fs::File;
use termion::{color, style};
use lazysort::{Sorted, SortedBy};
use regex::Regex;
use regex;

use red_master::RedMaster;
use red_buffer::RedBuffer;
use range::Range;

static SEL_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}";

#[derive(Debug, Clone)]
pub enum Action {
    Delete,  // Deletes a line
    Insert,  // Inserts text before a line
    Change,  // Change the content of a line
    Append,  // Append text at the end of a line

    Clear, // Clear the screen

    CopyTo(Range),   // Move a range from one place to another
    Substitute(String, String), // Substitute a by b

    Print,   // Print a range with line number
    Print_,   // Print a line

    BufList, // List all buffers
    BufChange(usize), // Change buffer
    BufNew, // New buffer
    BufDel(bool), // Delete buffer (force)

    Write(String),
    Edit(bool, String)
}

#[derive(Debug)]
pub enum ActionErr {
    OutOfBounds,
    IO,
    Regex,
    Other,
}

impl Action {
    pub fn apply(self, master: &mut RedMaster) -> Result<(), ActionErr> {
        match self {
            Action::Delete => {
                let file = master.curr_buf_mut();
                file.lines = file.clone().lines.into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| if !file.cursor.lines.contains(&i) { Some(line) } else { None })
                        .collect();
            }
            Action::Insert => {
                let file = master.curr_buf_mut();
                loop {
                    let mut to_insert = "".to_string();
                    print!("> ");
                    stdout().flush()?;
                    stdin().read_line(&mut to_insert)?;
                    to_insert = to_insert.trim_right_matches("\n").to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line, to_insert.clone())?;
                    }
                }
            }
            Action::Append => {
                let file = master.curr_buf_mut();
                let mut i = 1;
                loop {
                    let mut to_insert = "".to_string();
                    print!("a> ");
                    stdout().flush()?;
                    stdin().read_line(&mut to_insert)?;
                    to_insert = to_insert.trim_right_matches("\n").to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line + i, to_insert.clone())?;
                    }
                    i += 1;
                }
            }
            Action::Change => {
                let file = master.curr_buf_mut();
                for line in file.cursor.lines.clone().into_iter().sorted() {
                    let content = file.lines[line].clone();
                    let mut sel_chars = SEL_CHARS.to_string();
                    sel_chars.truncate(content.len() + 1);

                    println!("  {}", content);
                    println!("{}  {}{}", color::Fg(color::Cyan), sel_chars, style::Reset);

                    let mut targets;
                    while {
                        targets = "".to_string();
                        print!("T> ");
                        stdout().flush()?;
                        stdin().read_line(&mut targets)?;
                        targets = targets.trim().to_string();
                        if targets.len() == 0 {
                            return Ok(());
                        }
                        targets.len() != 2
                    } {}
                    let start = sel_chars.find(targets.chars().nth(0)?)?;
                    let end = sel_chars.find(targets.chars().nth(1)?)? + 1;

                    let mut text = "".to_string();
                    print!("c> ");
                    stdout().flush()?;
                    stdin().read_line(&mut text)?;
                    text = text.trim_right_matches("\n").to_string();

                    let line_before = file.lines[line].clone();
                    file.lines[line] = line_before[..start].to_string();
                    file.lines[line].push_str(&text);
                    if end < content.len() {
                        file.lines[line].push_str(&line_before[end..]);
                    }
                    file.saved = false;
                }
            }
            Action::Clear => {
                use std::io::stdout;

                print!("\x1B[2J\x1B[1;1H");
                stdout().flush().expect("Can't flush STDOUT");
            }
            Action::CopyTo(to) => {
                let file = master.curr_buf_mut();
                let lines_to_yank: Vec<String> = file.cursor.lines.clone().into_iter().sorted().map(|l| file.lines[l].clone()).collect();
                let res_lines: Vec<usize> = to.lines.into_iter().sorted().collect();
                let mut last_line: Option<usize> = None;
                for (i, line) in lines_to_yank.into_iter().enumerate() {
                    let res_pos = res_lines.get(i).map(|x| *x).unwrap_or_else(|| last_line.unwrap() + 1);
                    file.insert_line(res_pos, line)?;
                    last_line = Some(res_pos);
                }
            }
            Action::Substitute(pat, rep) => {
                let file = master.curr_buf_mut();
                let replacer: &str = &rep;
                let rpat = Regex::new(&pat)?;
                for i in file.cursor.lines.iter() {
                    let line = file.lines[*i].clone();
                    let replaced = rpat.replace_all(&line, replacer).into_owned();
                    file.lines[*i] = replaced;
                }
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
            }
            Action::BufDel(force) => {
                if !master.curr_buf().saved && !force {
                    eprintln!("Not saved!");
                    return Err(ActionErr::Other);
                }
                if master.buffers.len() == 1 {
                    return Err(ActionErr::OutOfBounds);
                }
                let idx = master.curr_buf_idx().clone();
                master.buffers.remove(idx);
                if idx > 0 {
                    master.change_buffer(idx - 1)?;
                }
            }
            Action::BufNew => {
                master.buffers.push(RedBuffer::empty());
                let buffers = master.buffers.len();
                master.change_buffer(buffers - 1)?;
            }
            Action::BufChange(i) => {
                master.change_buffer(i)?;
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
                    println!("Editing {} [{}]", path.trim(), file.lines.len());
                } else {
                    file.lines = vec![];
                    file.cursor = Range::empty();
                    println!("Editing {} [NEW]", path.trim());
                }
                file.saved = true;
                file.filename = Some(path.trim().to_string());
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
            }
            Action::Print_ => {
                let file = master.curr_buf_mut();
                for line in file.cursor.lines.iter().sorted() {
                    println!("{}", file.lines[*line]);
                }
            }
        }
        Ok(())
    }
}

impl From<io::Error> for ActionErr {
    fn from(_: io::Error) -> ActionErr {
        ActionErr::IO
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
