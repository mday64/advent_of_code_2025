mod parsing;
use parsing::parse_input;
use pathfinding::prelude::bfs;

pub fn part1(input: &str) -> usize {
    let (_, machines) = parse_input(input).expect("Invalid input");

    machines.iter()
        // .inspect(|machine| println!("{:?}", machine))
        .map(|machine| machine.configure_indicators())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (_, machines) = parse_input(input).expect("Invalid input");

    machines.iter()
        // .inspect(|machine| println!("{:?}", machine))
        .map(|machine| machine.configure_joltages())
        .inspect(|presses| eprintln!("{presses} presses"))
        .sum()
}

#[derive(Debug)]
pub struct Machine {
    indicators: u32,
    buttons: Vec<u32>,
    #[allow(unused)]
    joltages: Vec<u32>,
}

impl Machine {
    // Return the number of button presses to get the indicator lights
    // to match the machine definition.
    fn configure_indicators(&self) -> usize {
        // Starting state: all indicators off
        let start = 0u32;
        let success = |state: &u32| state == &self.indicators;
        let successors = |state: &u32| {
            self.buttons.iter()
                .map(|button| state ^ button)
                .collect::<Vec<_>>()
        };
        let path = bfs(&start, successors, success).unwrap();
        path.len() - 1
    }

    // Return the minimum number of button presses to get the joltages
    // to match the machine definition.
    fn configure_joltages(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 399);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 33);
    }
}
