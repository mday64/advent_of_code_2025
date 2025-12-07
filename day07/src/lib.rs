use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let starting_col = first_line.find('S').unwrap();
    let mut columns = HashSet::<usize>::default();
    columns.insert(starting_col);

    let mut splits = 0;

    for line in lines {
        for (splitter, _) in line.match_indices('^') {
            if columns.remove(&splitter) {
                splits += 1;
                columns.insert(splitter - 1);
                columns.insert(splitter + 1);
            }
        }
    }

    splits
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let starting_col = first_line.find('S').unwrap();
    let mut columns = HashMap::<usize, u64>::default();
    columns.insert(starting_col, 1);

    for line in lines {
        for (splitter, _) in line.match_indices('^') {
            if let Some(count) = columns.remove(&splitter) {
                *columns.entry(splitter - 1).or_default() += count;
                *columns.entry(splitter + 1).or_default() += count;
            }
        }
    }

    columns.values().sum()
}

pub fn both(input: &str) -> (u32, u64) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let starting_col = first_line.find('S').unwrap();
    let mut columns = HashMap::<usize, u64>::default();
    columns.insert(starting_col, 1);

    let mut splits = 0;

    for line in lines {
        for (splitter, _) in line.match_indices('^') {
            if let Some(count) = columns.remove(&splitter) {
                splits += 1;
                *columns.entry(splitter - 1).or_default() += count;
                *columns.entry(splitter + 1).or_default() += count;
            }
        }
    }

    (splits, columns.values().sum())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, both};
    
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

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 390684413472684);
    }

    #[test]
    fn test_both_example() {
        assert_eq!(both(EXAMPLE_INPUT), (21, 40));
    }

    #[test]
    fn test_both_full() {
        assert_eq!(both(FULL_INPUT), (1687, 390684413472684));
    }
}
