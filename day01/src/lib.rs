pub fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            match (&line[..1], line[1..].parse::<i32>().unwrap()) {
                ("L", num) => -num,
                ("R", num) => num,
                _ => unreachable!()
            }
        })
        .scan(50, |state, num| {
            *state = (*state + num).rem_euclid(100);
            Some(*state)
        })
        .filter(|num| *num == 0)
        .count()
}

pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut position: i32 = 50;
    for line in input.lines() {
        let amount: i32 = line[1..].parse().unwrap();
        assert!(amount > 0);

        if &line[..1] == "R" {
            position += amount;

            // See if we turned to or beyond 0
            while position >= 100 {
                position -= 100;
                result += 1;
            }
        } else if &line[..1] == "L" {
            // Left is a bit trickier.  If we end up at exactly 0, it does
            // not underflow.  And if we start at exactly 0, and turn less
            // than 100, then we should not count that underflow.

            if position == 0 {
                position = 100;
            }
            position -= amount;
            while position < 0 {
                position += 100;
                result += 1;
            }
            if position == 0 {
                result += 1;
            }
        }
    }

    result
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
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 6554);
    }
}
