use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut rolls: HashSet<(isize, isize)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((row as isize, col as isize));
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
    let mut rolls: HashSet<(isize, isize)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((row as isize, col as isize));
            }
        }
    }
    let initial_count = rolls.len();

    loop {
        // TODO: Rather than considering all rolls, could we just consider
        // neighbors of just-removed rolls?
        let removable: HashSet<(isize, isize)> = rolls.iter().filter(|(row, col)| {
            let num_neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
                .iter().filter(|(dr, dc)| rolls.contains(&(row + dr, col + dc))).count();
            num_neighbors < 4
        }).cloned().collect();

        if removable.is_empty() {
            break;
        }

        eprintln!("Removing {} rolls", removable.len());

        for roll in removable {
            rolls.remove(&roll);
        }
    }

    eprintln!("{} rolls remaining", rolls.len());

    initial_count - rolls.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
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
    static FULL_INPUT: &str = include_str!("../input.txt");

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
