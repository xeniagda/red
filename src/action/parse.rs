use nom::IResult;

use red_buffer::RedBuffer;
use action::Action;

use range::parse::parse_range;


pub fn parse_action<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt!(
        inp,

        flat_map!(tag!("d"), value!(Action::Delete)) |
        flat_map!(tag!("i"), value!(Action::Insert)) |
        flat_map!(tag!("c"), value!(Action::Change)) |
        flat_map!(tag!("a"), value!(Action::Append)) |
        flat_map!(tag!("p"), value!(Action::Print)) |
        flat_map!(tag!("P"), value!(Action::Print_)) |
        apply!(copy_to, ctx) |
        apply!(substitute, ctx) |
        apply!(write, ctx) |
        apply!(read, ctx)
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

        ws!(tag!("t")) >>
        to: apply!(parse_range, ctx) >>
        (Action::CopyTo(to))
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
