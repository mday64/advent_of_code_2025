use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .flatten()
        .filter(|num| {
            (num % 11 == 0 && (1..=9).contains(&(num / 11))) ||
            (num % 101 == 0 && (10..=99).contains(&(num / 101))) ||
            (num % 1001 == 0 && (100..=999).contains(&(num / 1001))) ||
            (num % 10001 == 0 && (1000..=9999).contains(&(num / 10001))) ||
            (num % 100001 == 0 && (10000..=99999).contains(&(num / 100001))) ||
            (num % 1000001 == 0 && (100000..=999999).contains(&(num / 1000001))) ||
            (num % 10000001 == 0 && (1000000..=9999999).contains(&(num / 10000001))) ||
            (num % 100000001 == 0 && (10000000..=99999999).contains(&(num / 100000001))) ||
            (num % 1000000001 == 0 && (100000000..=999999999).contains(&(num / 1000000001))) ||
            (num % 10000000001 == 0 && (1000000000..=9999999999).contains(&(num / 10000000001)))
        })
        // .inspect(|num| println!("num={num}"))
        .sum()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim_end()
        .split(',')
        .map(|s| {
            let mut ends = s.split('-').map(|num_str| num_str.parse::<u64>().unwrap());
            ends.next().unwrap() ..= ends.next().unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
    }
}
