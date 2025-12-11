use rustc_hash::FxHashMap;

//
// Find the number of distinct paths from node "you" to node "out".
//
// NOTE: I can't use pathfinding::directed::dfs::dfs_reach because it
// caches the nodes it has visited, and would only return "out" once.
pub fn part1(input: &str) -> u32 {
    let graph = parse_input(input);
    count_paths("you", "out", &graph)
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

fn count_paths(from: &str, to: &str, graph: &FxHashMap<&str, Vec<&str>>) -> u32 {
    let mut result = 0;
    // This feels like I'm declaring the closure wrong.  I shouldn't need &mut.
    dfs(from, graph, &mut |node| {
        if node == to {
            result += 1;
        }
    });
    result
}

fn dfs<T: FnMut(&str)>(start: &str, graph: &FxHashMap<&str, Vec<&str>>, visit: &mut T) {
    visit(start);
    for &neighbor in graph[start].iter() {
        dfs(neighbor, graph, visit);
    }
}

fn parse_input(input: &str) -> FxHashMap<&str, Vec<&str>> {
    let mut result: FxHashMap<&str, Vec<&str>> = input.lines()
    .map(|line| {
        let (node, rest) = line.split_once(": ").unwrap();
        let neighbors = rest.split_ascii_whitespace().collect();
        (node, neighbors)
    }).collect();
    result.insert("out", vec![]);
    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        let example = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        assert_eq!(part1(example), 5);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 643);
    }

    #[test]
    fn test_part2_example() {
        let example = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!(part2(example), "World");
    }
}
