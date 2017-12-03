pub mod parse;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Range {
    pub lines: HashSet<usize>
}

impl Range {
    pub fn new_with_line(line: usize) -> Range {
        let mut range = Range { lines: HashSet::new() };
        range.lines.insert(line);
        range
    }
    pub fn empty() -> Range {
        Range { lines: HashSet::new() }
    }
    pub fn offset(self, offset: isize) -> Range {
        let lines = self.lines.into_iter()
                .map(|x| x.wrapping_add(offset as usize))
                .collect();
        Range { lines: lines }
    }
}
