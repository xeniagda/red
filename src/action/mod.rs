pub mod parse;

use std::io::{stdin, stdout, Write, Read};
use std::fs::File;
use termion::{color, style};
use lazysort::{Sorted, SortedBy};

use red_file::RedFile;
use range::Range;

#[derive(Debug, Clone)]
pub enum Action {
    Delete,  // Deletes a line
    Insert,  // Inserts text before a line
    Change,  // Change the content of a line
    Append,  // Append text at the end of a line

    CopyTo(Range),   // Move a range from one place to another
    
    Print,   // Print a range with line number
    Print_,   // Print a line

    Write(String),
    Edit(String)
}

impl Action {
    pub fn apply(self, mut file: &mut RedFile) {
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
                    stdout().flush().unwrap();
                    stdin().read_line(&mut to_insert).unwrap();
                    to_insert = to_insert.trim().to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line, to_insert.clone());
                    }
                }
            }
            Action::Append => {
                loop {
                    let mut to_insert = "".to_string();
                    print!("> ");
                    stdout().flush().unwrap();
                    stdin().read_line(&mut to_insert).unwrap();
                    to_insert = to_insert.trim().to_string();
                    if to_insert == "." {
                        break;
                    }
                    for line in file.cursor.lines.clone().into_iter().sorted_by(|x, y| y.cmp(x)) {
                        file.insert_line(line + 1, to_insert.clone());
                    }
                }
            }
            Action::Change => {
                Action::Delete.apply(&mut file);
                Action::Insert.apply(&mut file);
            }
            Action::CopyTo(to) => {
                let lines_to_yank: Vec<String> = file.cursor.lines.clone().into_iter().sorted().map(|l| file.lines[l].clone()).collect();
                let res_lines: Vec<usize> = to.lines.into_iter().sorted().collect();
                let mut last_line: Option<usize> = None;
                for (i, line) in lines_to_yank.into_iter().enumerate() {
                    println!("Line {}: {} (res: {:?})", i, line, res_lines);
                    let res_pos = res_lines.get(i).map(|x| *x).unwrap_or_else(|| last_line.unwrap() + 1);
                    println!("Res pos: {}", res_pos);
                    file.insert_line(res_pos, line);
                    last_line = Some(res_pos);
                }
            }
            Action::Write(path) => {
                let mut out = File::create(path.trim()).unwrap();
                let mut first = true;
                for line in file.lines.iter() {
                    if !first {
                        out.write(&[10]).unwrap();
                    }
                    first = false;
                    out.write(line.bytes().collect::<Vec<u8>>().as_slice()).unwrap();
                }
            }
            Action::Edit(path) => {
                let mut f = File::open(path.trim()).unwrap();
                let mut content = String::new();
                f.read_to_string(&mut content).unwrap();

                file.lines = content.lines().map(|x| x.to_string()).collect();
                file.cursor = Range::empty();
            }
            Action::Print => {
                let mut next = None;
                let leading_digits = file.cursor.lines.clone().into_iter()
                        .map(|x| ((x + 1) as f32).log10().floor() as usize)
                        .max()
                        .unwrap_or(0) + 1;

                for line in file.cursor.lines.clone().into_iter().sorted() {
                    if next.is_some() && Some(line) != next {
                        println!("    ...");
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

    }
}
