mod parsing;
use parsing::parse_input;
use pathfinding::prelude::{bfs, dijkstra};

pub fn part1(input: &str) -> usize {
    let (_, machines) = parse_input(input).expect("Invalid input");

    machines.iter().map(|machine| machine.configure_indicators()).sum()
}

pub fn part2(input: &str) -> u32 {
    let (_, machines) = parse_input(input).expect("Invalid input");

    machines.iter().map(|machine| machine.configure_joltages()).sum()
}

// Button is a list of indicators that it toggles
pub type Button = Vec<u8>;

pub struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

impl Machine {
    // Return the number of button presses to get the indicator lights
    // to match the machine definition.
    fn configure_indicators(&self) -> usize {
        // Starting state: all indicators off
        let start: Vec<bool> = self.indicators.iter().map(|_| false).collect();
        let success = |state: &Vec<bool>| state == &self.indicators;
        let successors = |state: &Vec<bool>| {
            self.buttons.iter().map(|button| {
                let mut lights = state.clone();
                for &i in button {
                    lights[i as usize] = !lights[i as usize];
                }
                lights
            }).collect::<Vec<_>>()
        };
        let path = bfs(&start, successors, success).unwrap();
        path.len() - 1
    }

    // Return the number of button presses to get the joltage levels
    // to match the machine definition.
    fn configure_joltages(&self) -> u32 {
        // Starting state: all joltages are zero
        let start: Vec<u32> = self.joltages.iter().map(|_| 0).collect();
        let success = |state: &Vec<u32>| state == &self.joltages;
        let successors = |state: &Vec<u32>| {
            self.buttons.iter().filter_map(|button| {
                let mut joltages = state.clone();
                for &i in button {
                    joltages[i as usize] += 1;
                }
                if joltages.iter().zip(self.joltages.iter())
                    .all(|(v1, v2)| v1 <= v2)
                {
                    Some((joltages, 1))
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        };
        let (_path, cost) = dijkstra(&start, successors, success).unwrap();
        cost
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
