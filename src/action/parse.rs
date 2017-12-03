use nom::IResult;

use red_file::RedFile;
use action::Action;


pub fn parse_action<'a>(inp: &'a str, ctx: &RedFile) -> IResult<&'a str, Action> {
    alt!(
        inp,

        flat_map!(tag!("d"), value!(Action::Delete)) |
        flat_map!(tag!("i"), value!(Action::Insert)) |
        flat_map!(tag!("c"), value!(Action::Change)) |
        flat_map!(tag!("a"), value!(Action::Append)) |
        flat_map!(tag!("p"), value!(Action::Print)) |
        flat_map!(tag!("P"), value!(Action::Print_)) |
        apply!(write, ctx) |
        apply!(read, ctx)
        )
}

pub fn write<'a>(inp: &'a str, ctx: &RedFile) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        ws!(tag!("w")) >>
        name: is_not_s!("") >>
        ({ 
            Action::Write(name.to_string())
        })
        )
}

pub fn read<'a>(inp: &'a str, ctx: &RedFile) -> IResult<&'a str, Action> {
    do_parse!(
        inp,

        ws!(tag!("e")) >>
        name: is_not_s!("") >>
        ({ 
            Action::Edit(name.to_string())
        })
        )
}
