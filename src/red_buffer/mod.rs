use range::Range;
use action::ActionErr;
use std::collections::HashMap;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedBuffer {
    pub lines: Vec<String>,
    pub cursor: Range,
    pub marks: HashMap<Mark, Range>,
    pub filename: Option<String>,
    pub saved: bool
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Mark(String);

impl From<String> for Mark {
    fn from(x: String) -> Mark {
        if x == "" {
            Mark("'".into())
        } else {
            Mark(x)
        }
    }
}

impl RedBuffer {
    pub fn insert_line(&mut self, at: usize, line: String) -> Result<(), ActionErr>{
        if at > self.lines.len() {
            return Err(ActionErr::OutOfBounds);
        }
        self.lines.insert(at, line);
        self.cursor = self.clone().cursor.inserted_line(at);
        self.marks = self.marks.clone().into_iter().map(|(m, r)| (m, r.inserted_line(at))).collect();
        self.saved = false;
        Ok(())
    }
    pub fn empty() -> RedBuffer {
        RedBuffer {
            lines: vec![ "".into() ],
            cursor: Range::empty(),
            marks: HashMap::new(),
            filename: None,
            saved: true
        }
    }
}