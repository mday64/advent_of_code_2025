use std::collections::BinaryHeap;
use core::cmp::Reverse;
use rustc_hash::FxHashSet as HashSet;
use itertools::Itertools;
use parsing::parse_input;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
    z: u64
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Point {
    // Return the square of the Euclidean distance from self to other
    fn distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2) +
        self.y.abs_diff(other.y).pow(2) +
        self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pair<'a> {
    // Note: distance must be first so that pairs are sorted by distance
    distance: u64,
    p1: &'a Point,
    p2: &'a Point,
}

impl<'a> Pair<'a> {
    fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Pair { p1, p2, distance: p1.distance_to(p2) }
    }
}

pub fn part1(input: &str, num_connections: usize) -> usize {
    // Parse the input
    let points = parse_input(input);

    // Produce a list of all unique pairs of points, sorted by
    // distance between the points.  Sorted from largest distance
    // to smallest distance, so that the smallest can be .pop()'ed.
    let mut pairs = points.iter()
        .combinations(2)
        .map(|p| Pair::new(p[0], p[1]))
        .collect_vec() ;
    pairs.sort_unstable_by_key(|pair| pair.distance);
    pairs.reverse();

    // Create a list/set of components (circuits).
    // Initially, each point is in its own separate component.
    let mut circuits = points.iter().map(|point| {
        let mut set = HashSet::default();
        set.insert(point);
        set
    }).collect_vec();

    for _ in 0..num_connections {
        // Find and remove the shortest remaining distance -> two points
        let pair = pairs.pop().unwrap();

        // If the two points are in different circuits, then connect them.
        // The hard part here is getting mutable refences to both circuits
        // at the same time.
        let c1 = circuits.iter().position(|circuit| circuit.contains(pair.p1)).unwrap();
        if !circuits[c1].contains(pair.p2) {
            let c2 = circuits.iter().position(|circuit| circuit.contains(pair.p2)).unwrap();
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);
            let (part1, part2) = circuits.split_at_mut(c_max);
            part1[c_min].extend(part2[0].drain());
            circuits.remove(c_max);
        }
    }

    // Sort components/circuits by number of points
    // Return the sum of the three largest sizes
    let mut lengths = circuits.iter().map(|circuit| circuit.len()).collect_vec();
    lengths.sort_unstable();
    lengths.iter().rev().take(3).product()
}

pub fn part2(input: &str) -> u64 {
    // Parse the input
    let points = parse_input(input);

    // Produce a list of all unique pairs of points, sorted by
    // distance between the points.  Sorted from largest distance
    // to smallest distance, so that the smallest can be .pop()'ed.
    let mut pairs = points.iter()
        .combinations(2)
        .map(|p| Pair::new(p[0], p[1]))
        .collect_vec() ;
    pairs.sort_unstable_by_key(|pair| pair.distance);
    pairs.reverse();

    // Create a list/set of components (circuits).
    // Initially, each point is in its own separate component.
    let mut circuits = points.iter().map(|point| {
        let mut set = HashSet::default();
        set.insert(point);
        set
    }).collect_vec();

    loop {
        // Find and remove the shortest remaining distance -> two points
        let pair = pairs.pop().unwrap();

        // If the two points are in different circuits, then connect them.
        // The hard part here is getting mutable refences to both circuits
        // at the same time.
        let c1 = circuits.iter().position(|circuit| circuit.contains(pair.p1)).unwrap();
        if !circuits[c1].contains(pair.p2) {
            if circuits.len() == 2 {
                return pair.p1.x * pair.p2.x;
            }
            let c2 = circuits.iter().position(|circuit| circuit.contains(pair.p2)).unwrap();
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);
            let (part1, part2) = circuits.split_at_mut(c_max);
            part1[c_min].extend(part2[0].drain());
            circuits.remove(c_max);
        }
    }
}

pub fn both(input: &str, num_connections: usize) -> (usize, u64) {
    let mut result1 = 0;

    // Parse the input
    let points = parse_input(input);

    // Produce a list of all unique pairs of points, sorted by
    // distance between the points.
    let pairs = points.iter()
        .combinations(2)
        .map(|p| Reverse(Pair::new(p[0], p[1])))
        .collect_vec() ;
    let mut pairs = BinaryHeap::from(pairs);

    // Create a list/set of components (circuits).
    // Initially, each point is in its own separate component.
    let mut circuits = points.iter().map(|point| {
        let mut set = HashSet::default();
        set.insert(point);
        set
    }).collect_vec();

    let mut num_iterations = 0;
    while let Some(Reverse(pair)) = pairs.pop() {
        if num_iterations == num_connections {
            result1 = circuits.iter()
                .map(|circuit| circuit.len())
                .k_largest(3)
                .product();
        }
        num_iterations += 1;

        // If the two points are in different circuits, then connect them.
        // The hard part here is getting mutable refences to both circuits
        // at the same time.
        let c1 = circuits.iter().position(|circuit| circuit.contains(pair.p1)).unwrap();
        if !circuits[c1].contains(pair.p2) {
            if circuits.len() == 2 {
                // Connecting into a single circuit
                return (result1, pair.p1.x * pair.p2.x);
            }
            let c2 = circuits.iter().position(|circuit| circuit.contains(pair.p2)).unwrap();
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);
            let (part1, part2) = circuits.split_at_mut(c_max);
            part1[c_min].extend(part2[0].drain());
            circuits.remove(c_max);
        }
    }

    unreachable!()
}

mod parsing {
    use super::Point;
    use nom::{IResult, Parser, character::complete::{char, newline, u64}, combinator::all_consuming, multi::many1, sequence::terminated};

    fn point(input: &str) -> IResult<&str, Point> {
        let (remaining, (x, y, z)) = (
            terminated(u64, char(',')),
            terminated(u64, char(',')),
            terminated(u64, newline)
        ).parse(input)?;

        Ok((remaining, Point{x, y, z}))
    }

    pub fn parse_input(input: &str) -> Vec<Point> {
        let (_, points) = all_consuming(many1(point))
            .parse(input)
            .expect("Invalid input");
        points
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, both};
    
    static EXAMPLE_INPUT: &str = include_str!("../example.txt");
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT, 1000), 127551);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 25272);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 2347225200);
    }

    #[test]
    fn test_both_example() {
        assert_eq!(both(EXAMPLE_INPUT, 10), (40, 25272));
    }

    #[test]
    fn test_both_full() {
        assert_eq!(both(FULL_INPUT, 1000), (127551, 2347225200));
    }
}
