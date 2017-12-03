pub mod parse;

use std::io::{stdin, stdout, Write, Read};
use std::fs::File;
use lazysort::{Sorted, SortedBy};
use red_file::RedFile;
use range::Range;

#[derive(Debug, Clone)]
pub enum Action {
    Delete,  // Deletes a line
    Insert,  // Inserts text before a line
    Change,  // Change the content of a line
    Append,  // Append text at the end of a line
    
    Print,   // Print a range with line number
    Print_,   // Print a line

    Write(String),
    Edit(String)
}

impl Action {
    pub fn apply(self, file: &mut RedFile) {
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
                for line in file.cursor.lines.clone().into_iter().sorted() {
                    if next.is_some() && Some(line) != next {
                        println!("    ...");
                    }
                    println!("{}\t{}", line, file.lines[line]);
                    next = Some(line + 1);
                }
            }
            Action::Print_ => {
                for line in file.cursor.lines.iter().sorted() {
                    println!("{}", file.lines[*line]);
                }
            }
            _ => { }
        }

    }
}
