pub fn part1(_input: &str) -> String {
    "Hello".to_string()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "Hello, World!";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), "Hello");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
    }
}
