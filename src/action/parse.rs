use nom::IResult;

use red_buffer::RedBuffer;
use action::Action;

use range::parse::{parse_range, parse_usize};


pub fn parse_action<'a>(inp: &'a str, ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,

        apply!(paste, ctx) |
        apply!(yank, ctx) |
        apply!(delete, ctx) |
        flat_map!(tag!("i"), value!(Action::Insert)) |
        flat_map!(tag!("cl"), value!(Action::Clear)) |
        flat_map!(tag!("c"), value!(Action::Change)) |
        flat_map!(tag!("a"), value!(Action::Append)) |
        flat_map!(tag!("p"), value!(Action::Print)) |
        flat_map!(tag!("P"), value!(Action::Print_)) |
        flat_map!(tag!("bl"), value!(Action::BufList)) |
        apply!(insert, ctx) |
        apply!(append, ctx) |
        apply!(regs, ctx) |
        apply!(set_mark, ctx) |
        apply!(buf_change, ctx) |
        apply!(buf_new, ctx) |
        apply!(buf_del, ctx) |
        apply!(copy_to, ctx) |
        apply!(substitute, ctx) |
        apply!(write, ctx) |
        apply!(read, ctx)
        )
}

pub fn insert<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("I") >>
        text: is_not_s!("") >>
        (Action::InsertText(text.into()))
        )
}

pub fn append<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("A") >>
        text: is_not_s!("") >>
        (Action::AppendText(text.into()))
        )
}

pub fn delete<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("d") >>
        register: is_not_s!(" ") >>
        (Action::Delete(register.into()))
        )
}

pub fn yank<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("y") >>
        register: is_not_s!(" ") >>
        (Action::Yank(register.into()))
        )
}

pub fn paste<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("pa") >>
        register: is_not_s!(" ") >>
        (Action::Paste(register.into()))
        )
}

pub fn set_mark<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("m") >>
        mark: is_not_s!(" ") >>
        (Action::SetMark(mark.into()))
        )
}

pub fn regs<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,
        do_parse!(
            tag!("r") >>
            first: take_s!(1) >>
            rest: is_not_s!(" ") >>
            (Action::Registers(Some(format!("{}{}", first, rest))))
            ) |
        do_parse!(
            tag!("r") >>
            (Action::Registers(None))
            )

        )
}

pub fn buf_del<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,
        do_parse!(
            tag!("q!") >>
            (Action::BufDel(true))
            ) |
        do_parse!(
            tag!("q") >>
            (Action::BufDel(false))
            )
        )
}

pub fn buf_change<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        tag!("bc") >>
        buf: parse_usize >>
        (Action::BufChange(buf)
        )
    )
}

pub fn buf_new<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    alt_complete!(
        inp,
        do_parse!(
            tag!("bn") >>
            first: take_s!(1) >>
            rest: is_not_s!("") >>
            (Action::BufNew(Some(format!("{}{}", first, rest))))
            ) |
        do_parse!(
            tag!("bn") >>
            (Action::BufNew(None))
            )
        )
}

pub fn substitute<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
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

pub fn write<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        ws!(tag!("w")) >>
        name: is_not_s!("") >>
        (Action::Write(name.to_string()))
        )
}

pub fn read<'a>(inp: &'a str, _ctx: &RedBuffer) -> IResult<&'a str, Action> {
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