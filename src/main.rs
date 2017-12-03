#[macro_use]
extern crate nom;
extern crate regex;
extern crate lazysort;

mod range;
mod red_file;
mod action;

use nom::IResult;

use range::Range;
use range::parse::parse_range;
use action::parse::parse_action;
use red_file::RedFile;

fn main() {
    let mut file = 
            RedFile { lines: vec![
                "hello".to_string(),
                "world".to_string(),
                "jonathan".to_string(),
                "lööv".to_string()
            ], cursor: Range::empty() };
    loop {
        let mut line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        if line == line.trim() {
            break;
        }

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
                    action.apply(&mut file);
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


