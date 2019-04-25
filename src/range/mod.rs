pub mod parse;

use std::collections::HashSet;
use red_buffer::RedBuffer;

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
    pub fn removed_line(self, at: usize) -> Range {
        let res_lines: HashSet<usize> =
                self.lines.iter()
                    .map(|&l| if l >= at { l - 1 } else { l })
                    .collect();
        Range { lines: res_lines }
    }
    pub fn into_block(self, ctx: &RedBuffer) -> Range {
        Range { lines: self.lines.into_iter().flat_map(|l| line_to_block(l, ctx)).collect() }
    }
}

fn line_to_block(line: usize, ctx: &RedBuffer) -> Vec<usize> {
    if let Some(depth) = ctx.lines.get(line).and_then(|c| get_depth(c)) {
        let mut last = line + 1;
        while last < ctx.lines.len() {
            match get_depth(ctx.lines.get(last).unwrap()) {
                Some(d) if d <= depth => { break }
                _ => { }
            }
            last += 1;
        }
        return (line..=last).collect();
    } else {
        return vec![];
    }
}

fn get_depth(line: &str) -> Option<usize> {
    if line.trim().is_empty() {
        None
    } else {
        line.find(|x| x != ' ')
    }
}