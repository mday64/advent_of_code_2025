use crate::Machine;
use nom::{IResult, Parser, branch::alt, character::complete::{char, newline, u32}, combinator::all_consuming, multi::{many1, separated_list1}, sequence::delimited};

fn indicators(input: &str) -> IResult<&str, u32> {
    let (input, chars) = delimited(
        char('['),
        many1(alt((char('.'), char('#')))),
        char(']')
    ).parse(input)?;

    let mask = chars.into_iter().rev()
        .fold(0, |mask, ch| mask * 2 + if ch == '#' { 1 } else { 0 });

    Ok((input, mask))
}

fn button(input: &str) -> IResult<&str, u32> {
    let (input, nums) = delimited(
        char('('),
        separated_list1(char(','), u32),
        char(')')
    ).parse(input)?;

    let mask = nums.iter().fold(0u32, |mask, num| mask + (1 << num));
    Ok((input, mask))
}

fn buttons(input: &str) -> IResult<&str, Vec<u32>> {
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
