pub mod parse;

use std::io::{stdin, stdout, Write, Read};
use std::io;
use std::option;
use std::fs::File;
use termion::{color, style};
use lazysort::{Sorted, SortedBy};
use regex::Regex;
use regex;

use red_buffer::RedBuffer;
use range::Range;

static SEL_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}";

#[derive(Debug, Clone)]
pub enum Action {
    Delete,  // Deletes a line
    Insert,  // Inserts text before a line
    Change,  // Change the content of a line
    Append,  // Append text at the end of a line

    CopyTo(Range),   // Move a range from one place to another
    Substitute(String, String), // Substitute a by b

    Print,   // Print a range with line number
    Print_,   // Print a line

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
    pub fn apply(self, file: &mut RedBuffer) -> Result<(), ActionErr> {
        match self {
            Action::Delete => {
                file.lines = file.clone().lines.into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| if !file.cursor.lines.contains(&i) { Some(line) } else { None })
                        .collect();
            }
            Action::Insert => {
                loop {
                    let mut to_insert = "".to_string();
                    print!("> ");
                    stdout().flush()?;
                    stdin().read_line(&mut to_insert)?;
                    to_insert = to_insert.trim().to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line, to_insert.clone())?;
                    }
                }
            }
            Action::Append => {
                loop {
                    let mut to_insert = "".to_string();
                    print!("a> ");
                    stdout().flush()?;
                    stdin().read_line(&mut to_insert)?;
                    to_insert = to_insert.trim().to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line + 1, to_insert.clone())?;
                    }
                }
            }
            Action::Change => {
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
            Action::CopyTo(to) => {
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
                let replacer: &str = &rep;
                let rpat = Regex::new(&pat)?;
                for i in file.cursor.lines.iter() {
                    let line = file.lines[*i].clone();
                    let replaced = rpat.replace_all(&line, replacer).into_owned();
                    file.lines[*i] = replaced;
                }
            }
            Action::Write(n_path) => {
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
                let mut next = None;
                let leading_digits = file.cursor.lines.clone().into_iter()
                        .map(|x| ((x + 1) as f32).log10().floor() as usize)
                        .max()
                        .unwrap_or(0) + 1;

                for line in file.cursor.lines.clone().into_iter().sorted() {
                    if next.is_some() && Some(line) != next {
                        println!("{}    ...", color::Fg(color::Green));
                    }
                    println!("{4}{1:0$}{2} {3}", leading_digits, line, style::Reset, file.lines[line], color::Fg(color::Cyan));
                    next = Some(line + 1);
                }
            }
            Action::Print_ => {
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
