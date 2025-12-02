use std::ops::RangeInclusive;

//
// For the given ranges, find all numbers that consist of a sequence of
// digits repeated twice.  Return their sum.
//
pub fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .flatten()
        .filter(|num| {
            match num {
                10 ..= 99 =>                    // 2 digits; pattern is 1 digit
                    num % 11 == 0,
                1000 ..= 9999 =>                // 4 digits; pattern is 2 digits
                    num % 101 == 0,
                100000 ..= 999999 =>            // 6 digits; pattern is 3 digits
                    num % 1001 == 0,
                10000000 ..= 99999999 =>        // 8 digits; pattern is 4 digits
                    num % 10001 == 0,
                1000000000 ..= 9999999999 =>    // 10 digits; pattern is 5 digits
                    num % 100001 == 0,
                _ => false
            }
        })
        // .inspect(|num| println!("num={num}"))
        .sum()
}

//
// For the given ranges, find all numbers that consist of a sequence of
// digits repeated at least twice.  Return their sum.
//
// NOTE: The largest numbers in our full input have 10 digits.
//
pub fn part2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .flatten()
        .filter(|num| {
            match num {
                10 ..= 99 =>                    // 2 digits; pattern is 1 digit
                    num % 11 == 0,
                100 ..= 999 =>                  // 3 digits; pattern is 1 digit
                    num % 111 == 0,
                1000 ..= 9999 =>                // 4 digits; pattern is 2 digits
                    num % 101 == 0,
                10000 ..= 99999 =>              // 5 digits; pattern is 1 digit
                    num % 11111 == 0,
                100000 ..= 999999 =>            // 6 digits; pattern is 2 or 3 digits
                    num % 10101 == 0 || num % 1001 == 0,
                1000000 ..= 9999999 =>          // 7 digits; pattern is 1 digit
                    num % 1111111 == 0,
                10000000 ..= 99999999 =>        // 8 digits; pattern is 2 or 4 digits
                    num % 1010101 == 0 || num % 10001 == 0,
                100000000 ..= 999999999 =>      // 9 ditits; pattern is 3 digits
                    num % 1001001 == 0,
                1000000000 ..= 9999999999 =>    // 10 digits; pattern is 2 or 5 digits
                    num % 101010101 == 0 || num % 100001 == 0,
                _ => false
            }
        })
        // .inspect(|num| println!("num={num}"))
        .sum()
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
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 17077011375);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 36037497037);
    }}
