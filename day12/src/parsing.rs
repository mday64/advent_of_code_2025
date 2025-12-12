use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{char, newline, u32}, combinator::all_consuming, multi::{many1, separated_list1}, sequence::{delimited, separated_pair, terminated}};
use crate::{Region, Shape};

fn shape(input: &str) -> IResult<&str, Shape> {
    let (input, grid) = delimited(
        (u32, char(':'), newline),  // Ignore this part
        many1(terminated(many1(alt((char('#'), char('.')))), newline)),
        newline
    ).parse(input)?;
    Ok((input, Shape::new(grid)))
}

fn region(input: &str) -> IResult<&str, Region> {
    let (input, (width, length)) =
        terminated(separated_pair(u32, char('x'), u32), tag(": ")).parse(input)?;
    let (input, shapes) = separated_list1(char(' '), u32).parse(input)?;
    let (input, _) = newline.parse(input)?;
    Ok((input, Region{width, length, shapes}))
}

pub fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let (_input, (shapes, regions)) = all_consuming(
        (many1(shape), many1(region))
    ).parse(input).unwrap();

    (shapes, regions)
}
