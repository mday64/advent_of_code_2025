use rustc_hash::FxHashSet as HashSet;

// TODO: Instead of a HashSet, should I use a 2D array/grid?

pub fn part1(input: &str) -> usize {
    let mut rolls: HashSet<(i16, i16)> = HashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((row as i16, col as i16));
            }
        }
    }

    rolls.iter().filter(|(row, col)| {
        let num_neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
            .iter().filter(|(dr, dc)| rolls.contains(&(row + dr, col + dc))).count();
        num_neighbors < 4
    }).count()
}

pub fn part2(input: &str) -> usize {
    let mut rolls: HashSet<(i16, i16)> = HashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((row as i16, col as i16));
            }
        }
    }
    let initial_count = rolls.len();
    let mut removable = HashSet::<(i16, i16)>::default();

    loop {
        // TODO: Rather than considering all rolls, could we just consider
        // neighbors of just-removed rolls?
        removable.clear();
        removable.extend(rolls.iter().filter(|(row, col)| {
            let num_neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
                .iter().filter(|(dr, dc)| rolls.contains(&(row + dr, col + dc))).count();
            num_neighbors < 4
        }).cloned());

        if removable.is_empty() {
            break;
        }

        // eprintln!("Removing {} rolls", removable.len());

        for roll in removable.iter() {
            rolls.remove(&roll);
        }
    }

    // eprintln!("{} rolls remaining", rolls.len());

    initial_count - rolls.len()
}

pub fn parse_input(input: &str) -> HashSet<(i16, i16)> {
    let mut rolls: HashSet<(i16, i16)> = HashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((row as i16, col as i16));
            }
        }
    }
    rolls
}

pub mod part2_vec {
    use std::ops::{Index, IndexMut};

    #[derive(Debug, Clone)]
    pub struct Rolls {
        occupied: Vec<bool>,
        columns: usize,
        rows: usize,
    }

    impl Rolls {
        fn len(&self) -> usize {
            self.occupied.iter().filter(|item| **item).count()
        }

        fn count_neighbors(&self, row: usize, col: usize) -> u8 {
            self[(row-1, col-1)] as u8 +
            self[(row-1, col  )] as u8 +
            self[(row-1, col+1)] as u8 +
            self[(row  , col-1)] as u8 +
            self[(row  , col+1)] as u8 +
            self[(row+1, col-1)] as u8 +
            self[(row+1, col  )] as u8 +
            self[(row+1, col+1)] as u8
        }
    }

    impl Index<(usize, usize)> for Rolls {
        type Output = bool;
    
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            let (row, col) = index;
            debug_assert!(row < self.rows);
            debug_assert!(col < self.columns);
            &self.occupied[(row * self.columns + col) as usize]
        }
    }

    impl IndexMut<(usize, usize)> for Rolls {        
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            let (row, col) = index;
            debug_assert!(row < self.rows);
            debug_assert!(col < self.columns);
            &mut self.occupied[(row * self.columns + col) as usize]
        }
    }

    pub fn parse_input(input: &str) -> Rolls {
        let columns = (input.lines().next().unwrap().len() + 2) as usize;
        let rows = (input.lines().count() + 2) as usize;
        let mut occupied = Vec::new();
        occupied.resize(rows * columns, false);
        let mut rolls = Rolls { occupied, columns, rows };

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '@' {
                    rolls[(row+1, col+1)] = true;
                }
            }
        }

        rolls
    }

    pub fn part2_vec(input: &str) -> usize {
        let mut rolls = parse_input(input);
        let initial_count = rolls.len();

        let mut keep_going = true;
        while keep_going {
            keep_going = false;

            for row in 1..(rolls.rows-1) {
                for col in 1..(rolls.columns-1) {
                    if rolls[(row, col)] {
                        let neighbors = rolls.count_neighbors(row, col);
                        if neighbors < 4 {
                            rolls[(row, col)] = false;
                            keep_going = true;
                        }
                    }
                }
            }
        }

        initial_count - rolls.len()
    }

    #[cfg(test)]
    mod tests {
        use super::part2_vec;
        use crate::{EXAMPLE_INPUT, FULL_INPUT};

        #[test]
        fn test_part2_vec_example() {
            assert_eq!(part2_vec(EXAMPLE_INPUT), 43);
        }

        #[test]
        fn test_part2_vec_full() {
            assert_eq!(part2_vec(FULL_INPUT), 8739);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, EXAMPLE_INPUT, FULL_INPUT};
    
    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 1419);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 8739);
    }
}

pub static EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

pub static FULL_INPUT: &str = include_str!("../input.txt");
