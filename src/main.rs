#![feature(try_trait)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

extern crate rustyline;
extern crate regex;
extern crate lazysort;
extern crate termion;

mod range;
mod red_buffer;
mod action;
mod red_master;
mod readline;

use readline::read_line;

use std::env::args;

use nom::IResult;

use range::parse::parse_range;
use action::parse::parse_action;
use action::Action;
use action::ActionErr;
use red_master::RedMaster;

fn main() {
    let mut file = RedMaster::empty();

    for arg in args().skip(1) {
        if let Err(ActionErr::IO) = Action::Edit(true, arg).apply(&mut file) {
            eprintln!("Couldn't read file!");
        }
    }

    let mut last_line = "".to_string();

    loop {
        let line = read_line("");

        if let Err(_) = line {
            break;
        }

        let mut line = line.unwrap();

        if line.trim() == "!" {
            line = last_line.clone();
        }
        last_line = line.clone();

        let lineclone = line.clone();
        let range = parse_range(
                &lineclone,
                &file.curr_buf()
                );

        match range {
            IResult::Done(rest, range) => {
                file.curr_buf_mut().cursor = range;
                line = rest.to_string();
            }
            IResult::Error(e) => {
                eprintln!("Range error: {:?}", e);
                continue;
            }
            IResult::Incomplete(e) => {
                eprintln!("Range incomplete: {:?}", e);
                continue;
            }
        }

        while line.trim().len() > 0 {
            let lineclone = line.clone();
            let action = parse_action(&lineclone, &file.curr_buf());

            match action {
                IResult::Done(rest, action) => {
                    if let Err(x) = action.apply(&mut file) {
                        eprintln!("Application error: {:?}", x);
                    }
                    line = rest.to_string();
                }
                IResult::Error(e) => {
                    eprintln!("Action parse error: {:?} from {:?}", e, line);
                    break;
                }
                IResult::Incomplete(e) => {
                    eprintln!("Action incomplete: {:?}", e);
                    break;
                }
            }
        }
    }

}

