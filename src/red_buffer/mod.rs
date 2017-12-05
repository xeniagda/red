use range::Range;
use action::ActionErr;


#[derive(Debug, Clone)]
pub struct RedBuffer {
    pub lines: Vec<String>,
    pub cursor: Range,
    pub filename: Option<String>,
    pub saved: bool
}

impl RedBuffer {
    pub fn insert_line(&mut self, at: usize, line: String) -> Result<(), ActionErr>{
        if at > self.lines.len() {
            return Err(ActionErr::OutOfBounds);
        }
        self.lines.insert(at, line);
        self.cursor = Range {
            lines: self.clone().cursor.lines.into_iter()
                .map(|l| if l >= at { l + 1 } else { l })
                .collect()
        };
        self.saved = false;
        Ok(())
    }
    pub fn empty() -> RedBuffer {
        RedBuffer { lines: vec![], cursor: Range::empty(), filename: None, saved: true }
    }
}
