use nom::{IResult, Parser, character::complete::{char, newline, u64}, combinator::all_consuming, multi::many1, sequence::{separated_pair, terminated}};
use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let (_, points) = parse_input(input).expect("Invalid input");
    points.iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.area(p2))
        .max()
        .unwrap()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    all_consuming(
        many1(
            terminated(
                separated_pair(u64, char(','), u64)
                    .map(|(x,y)| Point { x, y }),
                newline
            )
        )
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 4782151432);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
    }
}
