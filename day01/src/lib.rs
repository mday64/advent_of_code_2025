pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    let mut position: i32 = 50;
    for line in input.lines() {
        let amount: i32 = line[1..].parse().unwrap();
        
        if &line[..1] == "R" {
            position = (position + amount) % 100;
        } else if &line[..1] == "L" {
            position = (position - amount) % 100;
        }
        if position < 0 {
            position += 100;
        }

        assert!(position >= 0);

        if position == 0 {
            result += 1;
        }
    }

    result
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 1120);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
    }
}
