use rustc_hash::FxHashMap;
use pathfinding::prelude::count_paths;

//
// Find the number of distinct paths from node "you" to node "out".
//
pub fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    count_paths("you", |node| graph[node].iter().cloned(), |&node| node == "out")
}

//
// Count the number of paths from "svr" to "out" that pass through 
// both "dac" and "fft" (in either order).
//
// This becomes counting the number of paths in:
//      svr -> dac -> fft -> out
// plus
//      svr -> fft -> dac -> out
//
// If there are no cycles, then there will either be a path from "dac"
// to "fft", or a path from "fft" to "dac", but not both.
//
pub fn part2(input: &str) -> usize {
    let graph = parse_input(input);
    let dac_fft = count_paths(
        "dac",
        |&node| graph[node].iter().cloned(),
        |&node| node == "fft");
    if dac_fft == 0 {
        // The only solutions are svr -> fft -> dac -> out

        let svr_fft = count_paths(
            "svr",
            |&node| graph[node].iter().cloned(),
            |&node| node == "fft");

        let fft_dac = count_paths(
            "fft",
            |&node| graph[node].iter().cloned(),
            |&node| node == "dac");

        let dac_out = count_paths(
            "dac",
            |&node| graph[node].iter().cloned(),
            |&node| node == "out");

        svr_fft * fft_dac * dac_out
    } else {
        // The only solutions are svr -> dac -> fft -> out
        
        // Count paths from svr to dac, without fft
        let svr_dac = count_paths(
            "svr",
            |&node| graph[node].iter().filter(|&&node| node != "fft").cloned(),
            |&node| node == "dac");
        
        let fft_out = count_paths(
            "fft",
            |&node| graph[node].iter().cloned(),
            |&node| node == "out");
        
        svr_dac * dac_fft * fft_out
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
        assert_eq!(part2(example), 2);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 417190406827152);
    }
}
