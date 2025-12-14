use std::iter::{FusedIterator, zip};

use crate::Machine;

pub fn configure_joltages(machine: &Machine) -> u32 {
    // For each joltage, figure out which buttons contribute to it.
    let mut equations = vec![vec![0u32; machine.buttons.len()]; machine.joltages.len()];
    for (button_num, button) in machine.buttons.iter().enumerate() {
        for joltage in button {
            equations[*joltage as usize][button_num] = 1;
        }
    }

    let min_presses = machine.joltages.iter().cloned().max().unwrap();
    
    for presses in min_presses.. {
        for partition in Partitions::new(machine.buttons.len(), presses) {
            let joltages: Vec<u32> = equations.iter()
                .map(|equation| dot_product(equation, &partition))
                .collect();
            if joltages == machine.joltages {
                return presses;
            }
        }
    }

    unreachable!()
}

fn dot_product(v1: &[u32], v2: &[u32]) -> u32 {
    zip(v1, v2).fold(0, |sum, (a, b)| sum + a * b)
}

struct Partitions {
    current: Vec<u32>,      // Count of presses per button
    num_buttons: usize,
    total_presses: u32,       // The total count of presses for all buttons
    done: bool,             // True if the iterator has been exhausted
}

impl Partitions {
    fn new(num_buttons: usize, total_presses: u32) -> Partitions {
        let mut current = vec![0; num_buttons];
        current[num_buttons - 1] = total_presses;

        Partitions { current, num_buttons, total_presses, done: false }
    }
}

impl Iterator for Partitions {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current.clone();

            // Move the iterator to the next state, or set done=true
            if self.current[0] == self.total_presses {
                self.done = true;
            } else {
                if *self.current.last().unwrap() > 0 {
                    self.current[self.num_buttons - 2] += 1;
                } else {
                    // Set the rightmost non-zero number to 0
                    let rightmost = self.current[0..self.num_buttons-1]
                        .iter()
                        .rposition(|v| *v != 0)
                        .unwrap();
                    assert!(rightmost > 0);
                    self.current[rightmost] = 0;

                    // Increment the number to its left
                    self.current[rightmost - 1] += 1;
                }

                self.current[self.num_buttons - 1] = self.total_presses -
                    self.current[0..self.num_buttons-1].iter().sum::<u32>();
            }

            Some(result)
        }
    }
}
impl FusedIterator for Partitions {}

#[cfg(test)]
mod part2_tests {
    use crate::{Machine, part2::{Partitions, configure_joltages}};

    #[test]
    fn partitions_3button_3presses() {
        let mut partitions = Partitions::new(3, 3);
        assert_eq!(partitions.next(), Some(vec![0, 0, 3]));
        assert_eq!(partitions.next(), Some(vec![0, 1, 2]));
        assert_eq!(partitions.next(), Some(vec![0, 2, 1]));
        assert_eq!(partitions.next(), Some(vec![0, 3, 0]));
        assert_eq!(partitions.next(), Some(vec![1, 0, 2]));
        assert_eq!(partitions.next(), Some(vec![1, 1, 1]));
        assert_eq!(partitions.next(), Some(vec![1, 2, 0]));
        assert_eq!(partitions.next(), Some(vec![2, 0, 1]));
        assert_eq!(partitions.next(), Some(vec![2, 1, 0]));
        assert_eq!(partitions.next(), Some(vec![3, 0, 0]));
        assert_eq!(partitions.next(), None);
        assert_eq!(partitions.next(), None);
    }

    #[test]
    fn part2_example_0() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]],
            joltages: vec![3,5,4,7],
        };

        assert_eq!(configure_joltages(&machine), 10);
    }

    #[test]
    fn part2_example_1() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![0,2,3,4], vec![2,3], vec![0,4], vec![0,1,2], vec![1,2,3,4]],
            joltages: vec![7,5,12,7,2],
        };

        assert_eq!(configure_joltages(&machine), 12);
    }

    #[test]
    fn part2_example_2() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![0,1,2,3,4], vec![0,3,4], vec![0,1,2,4,5], vec![1,2]],
            joltages: vec![10,11,11,5,10,5],
        };

        assert_eq!(configure_joltages(&machine), 11);
    }
}