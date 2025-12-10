use crate::{Button, Machine};
use nom::{IResult, Parser, branch::alt, character::complete::{char, newline, u8, u32}, combinator::all_consuming, multi::{many1, separated_list1}, sequence::delimited};

fn indicators(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        char('['),
        many1(alt((char('.').map(|_| false), char('#').map(|_| true)))),
        char(']')
    ).parse(input)
}

fn button(input: &str) -> IResult<&str, Button> {
    delimited(
        char('('),
        separated_list1(char(','), u8),
        char(')')
    ).parse(input)
}

fn buttons(input: &str) -> IResult<&str, Vec<Button>> {
    separated_list1(char(' '), button).parse(input)
}

fn joltages(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        char('{'),
        separated_list1(char(','), u32),
        char('}')
    ).parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (indicators, buttons, joltages, _newline)) = (
        indicators,
        delimited(char(' '), buttons, char(' ')),
        joltages,
        newline,
    ).parse(input)?;
    Ok((input, Machine{indicators, buttons, joltages}))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    all_consuming(many1(machine)).parse(input)
}
