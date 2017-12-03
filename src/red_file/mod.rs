use range::Range;

#[derive(Debug, Clone)]
pub struct RedFile {
    pub lines: Vec<String>,
    pub cursor: Range,
}

impl RedFile {
    pub fn insert_line(&mut self, at: usize, line: String) {
        self.lines.insert(at, line);
        self.cursor = Range {
            lines: self.clone().cursor.lines.into_iter()
                .map(|l| if l >= at { l + 1 } else { l })
                .collect()
        };
    }
}
