pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| {
        largest_num(line, 2)
    }).sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    input.lines().map(|line| {
        largest_num(line, 12)
    }).sum::<u64>()
}

//
// Return the largest number constructed from `digits` digits within
// `line`, in the order they appear within `line`.
//
// TODO: Try iterating over the characters within `line`, and keep the
// best answer so far.  Start with the first `digits` characters of `line`.
// Then for each following character, if it is larger than any of the
// characters remembered so far, remove the smallest remembered character
// and append the new character.  This would require only one pass over
// the characters in `line`.
//
fn largest_num(line: &str, digits: usize) -> u64 {
    let bytes = line.as_bytes();
    let length = bytes.len();
    let mut result = 0;
    let mut start = 0;

    for remaining in (0..digits).rev() {
        // Find the largest digit of `bytes` at offset `start_offset` or greater,
        // and not considering the last `remaining` values.
        let (index, byte) = find_largest(bytes, start, length-remaining);
        result = result * 10 + (byte - b'0') as u64;
        start = index + 1;
    }

    result
}

// Find the largest byte with index in start..end.  Return the index and byte.
// If the maximum value occurs multiple times, return the lowest such index.
fn find_largest(bytes: &[u8], start: usize, end: usize) -> (usize, u8) {
    debug_assert!(start < end);

    let mut largest_index = start;
    let mut largest_value = bytes[start];

    for index in (start+1)..end {
        if bytes[index] > largest_value {
            largest_index = index;
            largest_value = bytes[index];
        }
    }
    
    (largest_index, largest_value)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 17100);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 3121910778619);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 170418192256861);
    }
}
