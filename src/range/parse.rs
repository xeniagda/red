use std::collections::HashSet;
use nom::{IResult, digit};

use regex::Regex;
use red_buffer::RedBuffer;
use range::Range;

pub fn parse_range<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    alt_complete!(
        inp,

        do_parse!(
            ranges: separated_nonempty_list_complete!(
                tag!("+"),
                alt_complete!(
                    apply!(offset, ctx)
                    | apply!(expand, ctx)
                    | apply!(parse_one_range, ctx)
                    | delimited!(tag_s!("("), apply!(parse_range, ctx), tag_s!(")"))
                    )
                ) >>
            ( {
                let mut combined_ranges = HashSet::new();
                for range in ranges {
                    for line in range.lines {
                        combined_ranges.insert(line);
                    }
                }
                Range { lines: combined_ranges }
            } )
            )
        | value!(ctx.cursor.clone())

        )

}

fn parse_one_range<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    alt_complete!(
        inp,

        apply!(search, ctx)
        | apply!(range, ctx)
        | apply!(line_range, ctx)
        | apply!(special, ctx)
        )
}

fn line_range<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    do_parse!(
        inp,

        rel: apply!(line, ctx) >>
        (
            Range::new_with_line(rel)
        )
        )
}

fn offset<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    do_parse!(
        inp,

        range: apply!(parse_one_range, ctx) >>
        tag!("^") >>
        num: parse_isize >>
        ({
            range.offset(num)
        })
        )
}

fn expand<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    do_parse!(
        inp,

        range: apply!(parse_one_range, ctx) >>
        tag!("#") >>
        num: parse_isize >>
        ({
            let mut res = HashSet::new();
            if num < 0 {
                for i in 0..-num + 1 {
                    for x in range.lines.clone() {
                        res.insert(x.saturating_sub(i as usize));
                    }
                }
            } else {
                for i in 0..num + 1 {
                    for x in range.lines.clone() {
                        res.insert(x.wrapping_add(i as usize));
                    }
                }
            }
            Range { lines: res }
        })
        )
}

fn line<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, usize> {
    alt_complete!(
        inp,

        apply!(relative, ctx) |
        parse_usize |
        do_parse!(
            tag!("$") >>
            ( ctx.lines.len() - 1 )
        )
        )
}

fn relative<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, usize> {
    do_parse!(
        inp,

        range: parse_usize >>
        tag!("^") >>
        num: parse_isize >>
        ({
            range.wrapping_add(num as usize)
        })
        )
}


fn range<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    alt!(
        inp,
        do_parse!(
            start: apply!(line, ctx) >>
            tag_s!("-") >>
            end: apply!(line, ctx) >>
            ({
                Range { lines: (start..end+1).collect() }
            })
            ) |
        do_parse!(
            line: apply!(line, ctx) >>
            ({
                Range::new_with_line(line)
            })
            )
        )
}

fn special<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    alt!(
        inp,

        do_parse!(
            tag!("%") >>
            ( Range { lines: (0..ctx.lines.len()).collect() })
            ) |
        do_parse!(
            tag!(".") >>
            ( ctx.cursor.clone() )
            )
        )
}

named!(pub parse_usize<&str, usize>,
       flat_map!(
           recognize!(many1!(digit)),
           parse_to!(usize)
                )
       );

named!(pub parse_isize<&str, isize>,
       flat_map!(
           recognize!(preceded!(opt!(tag_s!("-")), many1!(digit))),
           parse_to!(isize)
           )
       );


fn search<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Range> {
    do_parse!(
        inp,

        tag_s!("/") >>
        pattern: is_not_s!("/") >>
        tag_s!("/") >>
        ({
            let re = Regex::new(pattern).unwrap();
            let mut matching = HashSet::new();
            for (i, line) in ctx.lines.iter().enumerate() {
                if re.find(&line).is_some() {
                    matching.insert(i);
                }
            }
            Range { lines: matching }
        })
        )
}
