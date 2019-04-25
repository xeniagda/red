
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
    let mut backlog = BACKLOG.lock().unwrap();
    if backlog.is_none() {
        *backlog = Some(Vec::new());
    }
    if let Some(ref mut bl) = *backlog {
        bl.push(cmd);
    }
}