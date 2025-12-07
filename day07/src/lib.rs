use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut columns = HashSet::from([first_line.find('S').unwrap()]);
    let mut splits = 0;

    for line in lines {
        for (splitter, _) in line.match_indices('^') {
            if columns.contains(&splitter) {
                splits += 1;
                columns.remove(&splitter);
                columns.insert(splitter - 1);
                columns.insert(splitter + 1);
            }
        }
    }

    splits
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut columns = vec![first_line.find('S').unwrap()];

    // NOTE: columns can and will contain duplicates if the beam ended up
    // in a given column because one path had it coming from the left, and
    // another path had it coming from the right.
    for line in lines {
        for (splitter, _) in line.match_indices('^') {
            columns = columns.into_iter().flat_map(|col| {
                if col == splitter {
                    vec![col - 1, col + 1]
                } else {
                    vec![col]
                }
            }).collect();
        }
    }

    columns.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = include_str!("../example.txt");
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 1687);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 40);
    }
}
