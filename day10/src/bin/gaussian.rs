use day10::parsing::parse_input;
use ndarray::prelude::*;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("Part 2 (Gaussian): {result}")
}

fn part2(input: &str) -> i32 {
    let (_, machines) = parse_input(input).expect("Invalid input");
    machines.into_iter()
        .map(|machine| {
            // Create a system of linear equations based on which buttons
            // map to which joltage.
            let mut equations = Array2::<i32>::zeros((machine.joltages.len(), machine.buttons.len()+1));
            for (b, button) in machine.buttons.iter().enumerate() {
                for &j in button.iter() {
                    equations[[j as usize, b]] = 1;
                }
            }
            let mut joltages = equations.slice_mut(s![.., -1]);
            for (dest, src) in joltages.iter_mut().zip(machine.joltages.iter()) {
                *dest = *src as i32;
            }

            println!("{equations:?}");

            // Do Gaussian elimination (or similar) to come up with a
            // solution (or family of solutions).

            1
        })
        .sum()
}

static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[test]
fn test_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 33);
}