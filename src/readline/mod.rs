
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::sync::Mutex;

lazy_static!{
    static ref EDITOR: Mutex<Editor<()>> = Mutex::new(Editor::new());
}

pub fn read_line(prompt: &str) -> Result<String, ReadlineError> {
    EDITOR.lock().unwrap().readline(prompt)
}
