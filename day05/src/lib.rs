use std::ops::RangeInclusive;
use nom::{IResult, Parser, bytes::complete::tag, character::complete::{newline, u64}, combinator::all_consuming, multi::many1, sequence::{separated_pair, terminated}};
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let (mut ranges, mut ids) = parse_input(input);
    ranges.sort_unstable_by_key(|range| *range.start());
    ranges = ranges.into_iter()
        .coalesce(|first, second| {
            if second.start() <= first.end() {
                Ok(*first.start() ..= *first.end().max(second.end()))
            } else {
                Err((first, second))
            }
        })
        .collect();
    let mut ranges = ranges.into_iter();

    ids.sort_unstable();

    let mut current_range = ranges.next().unwrap();
    let mut result = 0;
    for id in ids {
        while id > *current_range.end() {
            if let Some(next_range) = ranges.next() {
                current_range = next_range;
            } else {
                break;
            }
        }

        if current_range.contains(&id) {
            result += 1;
        }
    }

    result
}

pub fn part2(input: &str) -> usize {
    let (mut ranges, _ids) = parse_input(input);
    ranges.sort_by_key(|range| *range.start());

    // Combine overlapping ranges
    ranges.into_iter().coalesce(|first, second| {
        if second.start() <= first.end() {
            Ok(*first.start() ..= *first.end().max(second.end()))
        } else {
            Err((first, second))
        }
    })
    .map(|range| range.count())
    .sum()
}

pub fn both(input: &str) -> (usize, u64) {
    let (mut ranges, mut ids) = parse_input(input);
    ranges.sort_unstable_by_key(|range| *range.start());
    ranges = ranges.into_iter()
        .coalesce(|first, second| {
            if second.start() <= first.end() {
                Ok(*first.start() ..= *first.end().max(second.end()))
            } else {
                Err((first, second))
            }
        })
        .collect();

    let result2 = ranges.iter().map(|range| range.end() - range.start() + 1).sum();

    let mut ranges = ranges.into_iter();

    ids.sort_unstable();

    let mut current_range = ranges.next().unwrap();
    let mut result1 = 0;
    for id in ids {
        while id > *current_range.end() {
            if let Some(next_range) = ranges.next() {
                current_range = next_range;
            } else {
                break;
            }
        }

        if current_range.contains(&id) {
            result1 += 1;
        }
    }

    (result1, result2)
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (_, (ranges, ids)) = all_consuming(
        separated_pair(parse_ranges, newline, parse_ids)
    ).parse(input)
    .expect("Invalid input");

    (ranges, ids)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    terminated(separated_pair(u64, tag("-"), u64), newline)
        .map(|(first, second)| first..=second)
        .parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    many1(parse_range).parse(input)
}

fn parse_ids(input: &str) -> IResult<&str, Vec<u64>> {
    many1(terminated(u64, newline)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, both};
    
    static EXAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 874);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 348548952146313);
    }

    #[test]
    fn test_both_example() {
        assert_eq!(both(EXAMPLE_INPUT), (3, 14));
    }

    #[test]
    fn test_both_full() {
        assert_eq!(both(FULL_INPUT), (874, 348548952146313));
    }
}
