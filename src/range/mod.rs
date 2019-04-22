pub mod parse;

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn inserted_line(self, at: usize) -> Range {
        let mut res_lines: HashSet<usize> =
                self.lines.iter()
                    .map(|&l| if l >= at { l + 1 } else { l })
                    .collect();
        if self.lines.contains(&at) {
            res_lines.insert(at);
        }
        Range { lines: res_lines }
    }
}