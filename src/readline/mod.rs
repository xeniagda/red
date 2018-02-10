
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::sync::Mutex;

use config;

lazy_static!{
    static ref EDITOR: Mutex<Editor<()>> = Mutex::new(Editor::new());
    static ref BACKLOG: Mutex<Option<Vec<String>>> = Mutex::new(None);
}


pub fn read_line(prompt: &str) -> Result<String, ReadlineError> {
    let mut backlog = BACKLOG.lock().unwrap();
    let silent = config::CONF.lock().unwrap().silent;

    if let Some(ref mut backlog) = *backlog {
        if backlog.is_empty() {
            Err(ReadlineError::Eof)
        } else {
            let line = backlog.remove(0);
            if !silent {
                println!("{}", line);
            }

            Ok(line)
        }
    } else {
        EDITOR.lock().unwrap().readline(prompt)
    }

}

pub fn add_command(cmd: String) {
    let parts = split_escaped(cmd, ';', '\\');

    let mut backlog = BACKLOG.lock().unwrap();
    if backlog.is_none() {
        *backlog = Some(Vec::new());
    }
    if let Some(ref mut bl) = *backlog {
        bl.extend(parts);
    }
}

fn split_escaped(st: String, split: char, escape: char) -> Vec<String> {
    let mut res = vec![];

    let mut current_chunk = "".to_string();

    let mut chars = st.chars();

    while let Some(ch) = chars.next() {
        if ch == escape {
            if let Some(next) = chars.next() {
                if next == split || next == escape {
                    current_chunk.push(next);
                } else {
                    current_chunk.push(ch);
                    current_chunk.push(next);
                }
            } else {
                current_chunk.push(ch);
            }
        } else if ch == split {
            res.push(current_chunk);
            current_chunk = "".to_string();
        } else {
            current_chunk.push(ch);
        }
    }

    res.push(current_chunk);

    res
}


#[test]
fn test_split_escaped() {
    assert_eq!(split_escaped("Hello;world".into(), ';', '/'), vec!["Hello".to_string(), "world".to_string()]);
    assert_eq!(split_escaped("Hello/;world".into(), ';', '/'), vec!["Hello;world".to_string()]);
    assert_eq!(split_escaped("Hello//;world".into(), ';', '/'), vec!["Hello/".to_string(), "world".to_string()]);
    assert_eq!(split_escaped("He/llo;world".into(), ';', '/'), vec!["He/llo".to_string(), "world".to_string()]);
}
