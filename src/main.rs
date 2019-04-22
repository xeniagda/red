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
mod config;

use readline::{read_line, add_command};

use std::env::args;
use std::io::{stdin, Read};

use nom::IResult;

use range::parse::parse_range;
use action::parse::parse_action;
use action::Action;
use action::ActionErr;
use red_buffer::RedBuffer;
use red_master::RedMaster;
use range::Range;

fn main() {
    let mut file = RedMaster::empty();

    let mut args = args().skip(1); // Remove file path

    while let Some(arg) = args.next() {
        if arg == "-h" {
            println!("{}", include_str!("help.txt"));
            return;
        } else if arg == "-s" {
            config::CONF.lock().unwrap().silent = true;
        } else if arg == "-d" {
            if let Some(command_to_exec) = args.next() {
                add_command(command_to_exec);
            }
        } else if arg == "--" {
            // Read buffer from STDIN
            let mut data = String::new();
            if let Err(e) = stdin().read_to_string(&mut data) {
                eprintln!("Couldn't read from STDIN: {:?}", e);
            }

            let mut buf = RedBuffer::empty();
            buf.lines = data.lines().map(|x| x.to_string()).collect();
            buf.cursor = Range::empty();
            if !config::CONF.lock().unwrap().silent {
                println!("Editing [STDIN] [{}]", buf.lines.len());
            }

            file.buffers = vec![buf];
        } else if let Err(ActionErr::IO(err)) = Action::Edit(true, arg).apply(&mut file) {
            eprintln!("Couldn't read file! ({:?})", err);
        }
    }

    let mut last_line = "".to_string();

    let mut quitting = false;

    loop {
        let line = read_line("");

        if let Err(_) = line {
            if config::CONF.lock().unwrap().silent {
                break;
            }
            let buf = file.curr_buf();
            if !buf.saved {
                if quitting {
                    eprintln!("STDIN borked. Quitting");
                    break;
                }
                eprintln!("File not saved! Type q! to force quit.");
                quitting = true;
                continue;
            }
            break;
        }

        quitting = false;

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
            let action = parse_action(&lineclone.trim(), &file.curr_buf());

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