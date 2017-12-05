#![feature(try_trait)]

#[macro_use]
extern crate nom;
extern crate regex;
extern crate lazysort;
extern crate termion;

mod range;
mod red_buffer;
mod action;

use std::env::args;

use nom::IResult;

use range::Range;
use range::parse::parse_range;
use action::parse::parse_action;
use action::Action;
use action::ActionErr;
use red_buffer::RedBuffer;

fn main() {
    let mut file = RedBuffer { lines: vec![], cursor: Range::empty(), filename: None };
    for arg in args().skip(1) {
        if let Err(ActionErr::IO) = Action::Edit(arg).apply(&mut file) {
            eprintln!("Couldn't read file!");
        }
    }

    let mut last_line = "".to_string();

    loop {
        let mut line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        if line == line.trim() {
            break;
        }
        line = line.trim_right_matches("\n").to_string();

        if line.trim() == "!" {
            line = last_line.clone();
        }
        last_line = line.clone();

        let lineclone = line.clone();
        let range = parse_range(
                &lineclone,
                &file
                );

        match range {
            IResult::Done(rest, range) => {
                file.cursor = range;
                line = rest.to_string();
            }
            IResult::Error(e) => {
                eprintln!("Error: {:?}", e);
                continue;
            }
            IResult::Incomplete(e) => {
                eprintln!("Incomplete: {:?}", e);
                continue;
            }
        }

        while line.trim().len() > 0 {
            let lineclone = line.clone();
            let action = parse_action(&lineclone, &file);
            match action {
                IResult::Done(rest, action) => {
                    if let Err(x) = action.apply(&mut file) {
                        eprintln!("Err: {:?}", x);
                    }
                    line = rest.to_string();
                }
                IResult::Error(e) => {
                    eprintln!("Error: {:?} from {:?}", e, line);
                    break;
                }
                IResult::Incomplete(e) => {
                    eprintln!("Incomplete: {:?}", e);
                    break;
                }
            }
        }
    }
    
}


