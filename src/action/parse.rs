use nom::IResult;

use red_buffer::RedBuffer;
use action::Action;

use range::parse::{parse_range, parse_usize};


pub fn parse_action<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,

        flat_map!(tag!("d"), value!(Action::Delete)) |
        flat_map!(tag!("i"), value!(Action::Insert)) |
        flat_map!(tag!("cl"), value!(Action::Clear)) |
        flat_map!(tag!("c"), value!(Action::Change)) |
        flat_map!(tag!("a"), value!(Action::Append)) |
        flat_map!(tag!("p"), value!(Action::Print)) |
        flat_map!(tag!("P"), value!(Action::Print_)) |
        flat_map!(tag!("bl"), value!(Action::BufList)) |
        flat_map!(tag!("bn"), value!(Action::BufNew)) |
        apply!(buf_change, ctx) |
        apply!(buf_del, ctx) |
        apply!(copy_to, ctx) |
        apply!(substitute, ctx) |
        apply!(write, ctx) |
        apply!(read, ctx)
        )
}


pub fn buf_del<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,
        do_parse!(
            tag!("bd!") >>
            (Action::BufDel(true))
            ) |
        do_parse!(
            tag!("bd") >>
            (Action::BufDel(false))
            )
        )
}

pub fn buf_change<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("bc") >>
        buf: parse_usize >>
        (Action::BufChange(buf)
        )
    )
}

pub fn substitute<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("s/") >>
        pattern: is_not_s!("/") >>
        tag!("/") >>
        replace: is_not_s!("/") >>
        (Action::Substitute(pattern.to_string(), replace.to_string()))
        )
}

pub fn copy_to<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        ws!(tag_s!("t")) >>
        to: apply!(parse_range, ctx) >>
        ( Action::CopyTo(to) )
        )
}

pub fn write<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        ws!(tag!("w")) >>
        name: is_not_s!("") >>
        (Action::Write(name.to_string()))
        )
}

pub fn read<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt!(
        inp,


        do_parse!(
            ws!(tag!("e!")) >>
            name: is_not_s!("") >>
            (Action::Edit(true, name.to_string()))
            ) |
        do_parse!(
            ws!(tag!("e")) >>
            name: is_not_s!("") >>
            (Action::Edit(false, name.to_string()))
            )
        )
}
