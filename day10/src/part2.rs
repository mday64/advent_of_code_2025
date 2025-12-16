use std::iter::{FusedIterator, zip};

use crate::Machine;

pub fn configure_joltages(machine: &Machine) -> u32 {
    let num_buttons = machine.buttons.len();

    // For each joltage, figure out which buttons contribute to it.
    let mut equations = vec![vec![0u32; num_buttons]; machine.joltages.len()];
    for (button_num, button) in machine.buttons.iter().enumerate() {
        for joltage in button {
            equations[*joltage as usize][button_num] = 1;
        }
    }

    let min_presses = machine.joltages.iter().cloned().max().unwrap();
    
    for presses in min_presses.. {
        // Calculate the minimum and maximum presses for each button
        let max_per_button = machine.buttons.iter().map(|button| {
            button.iter().map(|index| {
                machine.joltages[*index as usize]
            }).min().unwrap()
        }).collect::<Vec<u32>>();
        let total_max_presses = max_per_button.iter().sum::<u32>();
        let min_per_button = (0..num_buttons).map(|index| {
            let total_other_max = total_max_presses - max_per_button[index];
            if total_other_max > presses {
                0
            } else {
                presses - total_other_max
            }
        }).collect::<Vec<u32>>();

        // Try the various combinations of presses
        for partition in Partitions::new(min_per_button, max_per_button, presses) {
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
    min_presses: Vec<u32>,  // Minimum presses per button
    max_presses: Vec<u32>,  // Maximum optional presses per button
    num_buttons: usize,
    //total_presses: u32,     // The total count of optional presses for all buttons
    done: bool,             // True if the iterator has been exhausted
}

impl Partitions {
    fn new(min_presses: Vec<u32>, max_presses: Vec<u32>, total_presses: u32) -> Partitions {
        let num_buttons = min_presses.len();

        // Construct current[] to distribute total_presses according to
        // the min_presses and max_presses vectors, with any excess above
        // min_presses going to the last entries (as limited by max_presses).
        let mut current = min_presses.clone();
        let mut remaining_presses = total_presses - min_presses.iter().sum::<u32>();
        let mut index = min_presses.len() - 1;
        while remaining_presses > 0 {
            if current[index] + remaining_presses > max_presses[index] {
                current[index] = max_presses[index];
                remaining_presses -= max_presses[index] - min_presses[index];
            } else {
                current[index] += remaining_presses;
                remaining_presses = 0;
            }
            index -= 1;
        }
        Partitions { current, min_presses, max_presses, num_buttons, /*total_presses,*/ done: false }
    }
}

impl Iterator for Partitions {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current.clone();

            // Move the iterator to the next state, or set done=true.
            // Buttons with larger indices should vary fastest.
            let mut non_minimum = false;
            let mut non_maximum: Option<usize> = None;

            for index in (0..self.num_buttons).rev() {
                if non_minimum && self.current[index] < self.max_presses[index] {
                    non_maximum = Some(index);
                    break;
                }

                // NOTE: This check has to come AFTER the one above, or else
                // non_minimum and non_maximum could be set to the same index,
                // which would be wrong (and lead to the same partition being
                // returned forever).
                if self.current[index] > self.min_presses[index] {
                    non_minimum = true;
                }
            }

            if let Some(max_index) = non_maximum {
                self.current[max_index] += 1;

                // Subtract one press from buttons to the right of max_index.
                // Redistribute them as far to the right as possible.
                let mut presses = self.current[max_index+1..].iter().sum::<u32>() - 1;
                for index in (max_index+1..self.num_buttons).rev() {
                    if presses > self.max_presses[index] {
                        self.current[index] = self.max_presses[index];
                        presses -= self.max_presses[index];
                    } else {
                        self.current[index] = presses;
                        presses = 0;
                    }
                }
            } else {
                self.done = true;
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
        let mut partitions = Partitions::new(vec![0,0,0], vec![3,3,3], 3);
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
    fn partitions_example2() {
        // Using per-button minimums and maximums from the example.
        let mut partitions = Partitions::new(vec![0,1,0,0,0], vec![2,7,2,5,2], 12);
        assert_eq!(partitions.next(), Some(vec![0,3,2,5,2]));
        assert_eq!(partitions.next(), Some(vec![0,4,1,5,2]));
        assert_eq!(partitions.next(), Some(vec![0,4,2,4,2]));
        assert_eq!(partitions.next(), Some(vec![0,4,2,5,1]));
        assert_eq!(partitions.next(), Some(vec![0,5,0,5,2]));
        assert_eq!(partitions.next(), Some(vec![0,5,1,4,2]));
        assert_eq!(partitions.next(), Some(vec![0,5,1,5,1]));
        assert_eq!(partitions.next(), Some(vec![0,5,2,3,2]));
        assert_eq!(partitions.next(), Some(vec![0,5,2,4,1]));
        assert_eq!(partitions.next(), Some(vec![0,5,2,5,0]));
        assert_eq!(partitions.next(), Some(vec![0,6,0,4,2]));
        assert_eq!(partitions.next(), Some(vec![0,6,0,5,1]));
        assert_eq!(partitions.next(), Some(vec![0,6,1,3,2]));
        assert_eq!(partitions.next(), Some(vec![0,6,1,4,1]));
        assert_eq!(partitions.next(), Some(vec![0,6,1,5,0]));
        assert_eq!(partitions.next(), Some(vec![0,6,2,2,2]));
        assert_eq!(partitions.next(), Some(vec![0,6,2,3,1]));
        assert_eq!(partitions.next(), Some(vec![0,6,2,4,0]));
        assert_eq!(partitions.next(), Some(vec![0,7,0,3,2]));
        assert_eq!(partitions.next(), Some(vec![0,7,0,4,1]));
        assert_eq!(partitions.next(), Some(vec![0,7,0,5,0]));
        assert_eq!(partitions.next(), Some(vec![0,7,1,2,2]));
        assert_eq!(partitions.next(), Some(vec![0,7,1,3,1]));
        assert_eq!(partitions.next(), Some(vec![0,7,1,4,0]));
        assert_eq!(partitions.next(), Some(vec![0,7,2,1,2]));
        assert_eq!(partitions.next(), Some(vec![0,7,2,2,1]));
        assert_eq!(partitions.next(), Some(vec![0,7,2,3,0]));
        assert_eq!(partitions.next(), Some(vec![1,2,2,5,2]));
        assert_eq!(partitions.next(), Some(vec![1,3,1,5,2]));
        assert_eq!(partitions.next(), Some(vec![1,3,2,4,2]));
        assert_eq!(partitions.next(), Some(vec![1,3,2,5,1]));
        assert_eq!(partitions.next(), Some(vec![1,4,0,5,2]));
        assert_eq!(partitions.next(), Some(vec![1,4,1,4,2]));
        assert_eq!(partitions.next(), Some(vec![1,4,1,5,1]));
        assert_eq!(partitions.next(), Some(vec![1,4,2,3,2]));
        assert_eq!(partitions.next(), Some(vec![1,4,2,4,1]));
        assert_eq!(partitions.next(), Some(vec![1,4,2,5,0]));
        assert_eq!(partitions.next(), Some(vec![1,5,0,4,2]));
        assert_eq!(partitions.next(), Some(vec![1,5,0,5,1]));
        assert_eq!(partitions.next(), Some(vec![1,5,1,3,2]));
        assert_eq!(partitions.next(), Some(vec![1,5,1,4,1]));
        assert_eq!(partitions.next(), Some(vec![1,5,1,5,0]));
        assert_eq!(partitions.next(), Some(vec![1,5,2,2,2]));
        assert_eq!(partitions.next(), Some(vec![1,5,2,3,1]));
        assert_eq!(partitions.next(), Some(vec![1,5,2,4,0]));
        assert_eq!(partitions.next(), Some(vec![1,6,0,3,2]));
        assert_eq!(partitions.next(), Some(vec![1,6,0,4,1]));
        assert_eq!(partitions.next(), Some(vec![1,6,0,5,0]));
        assert_eq!(partitions.next(), Some(vec![1,6,1,2,2]));
        assert_eq!(partitions.next(), Some(vec![1,6,1,3,1]));
        assert_eq!(partitions.next(), Some(vec![1,6,1,4,0]));
        assert_eq!(partitions.next(), Some(vec![1,6,2,1,2]));
        assert_eq!(partitions.next(), Some(vec![1,6,2,2,1]));
        assert_eq!(partitions.next(), Some(vec![1,6,2,3,0]));
        assert_eq!(partitions.next(), Some(vec![1,7,0,2,2]));
        assert_eq!(partitions.next(), Some(vec![1,7,0,3,1]));
        assert_eq!(partitions.next(), Some(vec![1,7,0,4,0]));
        assert_eq!(partitions.next(), Some(vec![1,7,1,1,2]));
        assert_eq!(partitions.next(), Some(vec![1,7,1,2,1]));
        assert_eq!(partitions.next(), Some(vec![1,7,1,3,0]));
        assert_eq!(partitions.next(), Some(vec![1,7,2,0,2]));
        assert_eq!(partitions.next(), Some(vec![1,7,2,1,1]));
        assert_eq!(partitions.next(), Some(vec![1,7,2,2,0]));
        assert_eq!(partitions.next(), Some(vec![2,1,2,5,2]));
        assert_eq!(partitions.next(), Some(vec![2,2,1,5,2]));
        assert_eq!(partitions.next(), Some(vec![2,2,2,4,2]));
        assert_eq!(partitions.next(), Some(vec![2,2,2,5,1]));
        assert_eq!(partitions.next(), Some(vec![2,3,0,5,2]));
        assert_eq!(partitions.next(), Some(vec![2,3,1,4,2]));
        assert_eq!(partitions.next(), Some(vec![2,3,1,5,1]));
        assert_eq!(partitions.next(), Some(vec![2,3,2,3,2]));
        assert_eq!(partitions.next(), Some(vec![2,3,2,4,1]));
        assert_eq!(partitions.next(), Some(vec![2,3,2,5,0]));
        assert_eq!(partitions.next(), Some(vec![2,4,0,4,2]));
        assert_eq!(partitions.next(), Some(vec![2,4,0,5,1]));
        assert_eq!(partitions.next(), Some(vec![2,4,1,3,2]));
        assert_eq!(partitions.next(), Some(vec![2,4,1,4,1]));
        assert_eq!(partitions.next(), Some(vec![2,4,1,5,0]));
        assert_eq!(partitions.next(), Some(vec![2,4,2,2,2]));
        assert_eq!(partitions.next(), Some(vec![2,4,2,3,1]));
        assert_eq!(partitions.next(), Some(vec![2,4,2,4,0]));
        assert_eq!(partitions.next(), Some(vec![2,5,0,3,2]));
        assert_eq!(partitions.next(), Some(vec![2,5,0,4,1]));
        assert_eq!(partitions.next(), Some(vec![2,5,0,5,0]));
        assert_eq!(partitions.next(), Some(vec![2,5,1,2,2]));
        assert_eq!(partitions.next(), Some(vec![2,5,1,3,1]));
        assert_eq!(partitions.next(), Some(vec![2,5,1,4,0]));
        assert_eq!(partitions.next(), Some(vec![2,5,2,1,2]));
        assert_eq!(partitions.next(), Some(vec![2,5,2,2,1]));
        assert_eq!(partitions.next(), Some(vec![2,5,2,3,0]));
        assert_eq!(partitions.next(), Some(vec![2,6,0,2,2]));
        assert_eq!(partitions.next(), Some(vec![2,6,0,3,1]));
        assert_eq!(partitions.next(), Some(vec![2,6,0,4,0]));
        assert_eq!(partitions.next(), Some(vec![2,6,1,1,2]));
        assert_eq!(partitions.next(), Some(vec![2,6,1,2,1]));
        assert_eq!(partitions.next(), Some(vec![2,6,1,3,0]));
        assert_eq!(partitions.next(), Some(vec![2,6,2,0,2]));
        assert_eq!(partitions.next(), Some(vec![2,6,2,1,1]));
        assert_eq!(partitions.next(), Some(vec![2,6,2,2,0]));
        assert_eq!(partitions.next(), Some(vec![2,7,0,1,2]));
        assert_eq!(partitions.next(), Some(vec![2,7,0,2,1]));
        assert_eq!(partitions.next(), Some(vec![2,7,0,3,0]));
        assert_eq!(partitions.next(), Some(vec![2,7,1,0,2]));
        assert_eq!(partitions.next(), Some(vec![2,7,1,1,1]));
        assert_eq!(partitions.next(), Some(vec![2,7,1,2,0]));
        assert_eq!(partitions.next(), Some(vec![2,7,2,0,1]));
        assert_eq!(partitions.next(), Some(vec![2,7,2,1,0]));
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