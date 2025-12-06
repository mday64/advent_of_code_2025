use nom::{IResult, Parser, branch::alt, character::{complete::{char, multispace0, multispace1, newline, u32}}, combinator::{all_consuming, opt}, multi::{many1, separated_list1}, sequence::delimited};

pub fn part1(input: &str) -> u32 {
    let (numbers, operators) = parse_input(input);
    42
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<char>) {
    let (_, (numbers, operators)) = all_consuming(
        (many1(number_row), operator_row)
    ).parse(input).expect("Invalid input");

    (numbers, operators)
}

fn number_row(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        multispace0,
        separated_list1(multispace1, u32),
        // NOTE: The input numbers are right-aligned, so there are no spaces before the newline
        newline
    ).parse(input)
}

fn operator_row(input: &str) -> IResult<&str, Vec<char>> {
    delimited(
        multispace0,
        separated_list1(multispace1, alt((char('*'), char('+')))),
        opt(multispace0)
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
    }
}
