pub mod parsing;
mod part2;
use parsing::parse_input;
use part2::configure_joltages;
use pathfinding::prelude::bfs;
// use rayon::prelude::*;

pub fn part1(input: &str) -> usize {
    let (_, machines) = parse_input(input).expect("Invalid input");

    machines.iter()
        // .inspect(|machine| println!("{:?}", machine))
        .map(|machine| {
            // Convert the indicators vector to a bit mask
            let indicators: u32 = machine.indicators.iter()
                .rev()
                .fold(0, |mask, &ch| {
                    mask * 2 + if ch == '#' { 1 } else { 0 }
                });

            // Convert each button from a vector to a bit mask
            let buttons = machine.buttons.iter().map(|button | {
                button.iter().fold(0, |mask, index| {
                    mask + (1 << index)
                })
            }).collect::<Vec<u32>>();

            // Starting state: all indicators off
            let start = 0u32;
            let success = |state: &u32| state == &indicators;
            let successors = |state: &u32| {
                buttons.iter()
                    .map(|button| state ^ button)
                    .collect::<Vec<_>>()
            };
            let path = bfs(&start, successors, success).unwrap();
            path.len() - 1
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (_, machines) = parse_input(input).expect("Invalid input");

    // let enumerated = machines.into_iter().enumerate().collect::<Vec<_>>();
    // enumerated.par_iter()
    //     .map(|(index, machine)| (index, configure_joltages(machine)))
    //     .inspect(|(index, presses)| println!("{index}: {presses} presses"))
    //     .map(|(_index, presses)| presses)
    //     .sum()

    machines.iter()
        .map(configure_joltages)
        .sum()
}

#[derive(Debug)]
pub struct Machine {
    pub indicators: Vec<char>,
    pub buttons: Vec<Vec<u32>>,
    #[allow(unused)]
    pub joltages: Vec<u32>,
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

    #[test]
    #[ignore = "Takes too long"]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 15631);
    }

    // Commit c7c9293 took 1784 seconds (just under 30 minutes).
    // Commit 02aa584 took 9.3 seconds
    // Commit afa3c35 (DFS, without rayon) took 65µs
    // Commit ??????? (DFS, better pruning) took 52 µs
    #[test]
    fn test_part2_full_line120() {
        let input = "[.###...] (0,2,3,4,6) (0,1,3,4) (0,1,2,4,5,6) (0,2,3,5) (1,5,6) {40,182,28,34,24,186,176}\n";
        assert_eq!(part2(input), 204);
    }
}
