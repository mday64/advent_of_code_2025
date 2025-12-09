use std::collections::BinaryHeap;
use nom::{IResult, Parser, character::complete::{char, newline, u64}, combinator::all_consuming, multi::many1, sequence::{separated_pair, terminated}};
use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let (_, points) = parse_input(input).expect("Invalid input");
    points.iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.area(p2))
        .max()
        .unwrap()
}

//
// Now, the list of points describes a region (much like Quickdraw).
// We must find the largest rectangle contained by the region.
//
// Note that the points are listed in order around the outside of the
// region (with the first and last points being connected).
//
// I think the solution is to consider all rectangles, filter out any
// where a line from the region's perimeter passes through the middle
// of the rectangle, and return the maximum area.
//
pub fn part2(input: &str) -> u64 {
    let (_, points) = parse_input(input).expect("Invalid input");
    points.iter()
        .tuple_combinations()
        .filter_map(|(p1, p2)| {
            let rect = Rect::new(p1, p2);
            if rect.contained_within_region(&points) {
                Some(rect.area())
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

pub fn part2_heap(input: &str) -> u64 {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct HeapRect<'a> {
        area: u64,
        p1: &'a Point,
        p2: &'a Point,
    }

    let (_, points) = parse_input(input).expect("Invalid input");
    let heap = points.iter()
        .tuple_combinations()
        .map(|(p1, p2)| HeapRect { area: p1.area(p2), p1, p2 })
        .collect_vec();
    let mut heap = BinaryHeap::from(heap);

    while let Some(rect) = heap.pop() {
        if Rect::new(rect.p1, rect.p2).contained_within_region(&points) {
            return rect.area;
        }
    }

    unreachable!()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

struct Rect {
    top: u64,
    left: u64,
    bottom: u64,
    right: u64,
}

impl Rect {
    fn new(p1: &Point, p2: &Point) -> Rect {
        let top = p1.y.min(p2.y);
        let bottom = p1.y.max(p2.y);
        let left = p1.x.min(p2.x);
        let right = p1.x.max(p2.x);

        Rect { top, left, bottom, right }
    }

    fn area(&self) -> u64 {
        (self.bottom - self.top + 1) * (self.right - self.left + 1)
    }

    //
    // Return true if the line between the two points goes through
    // the interior of self.  If the line merely overlaps one of
    // the edges of self, then return false.
    //
    fn interior_intersects(&self, p1: &Point, p2: &Point) -> bool {
        if (p1.x <= self.left && p2.x <= self.left) ||
           (p1.x >= self.right && p2.x >= self.right) ||
           (p1.y <= self.top && p2.y <= self.top) ||
           (p1.y >= self.bottom && p2.y >= self.bottom)
        {
            return false;
        } else {
            return true;
        }
    }

    //
    // Return true if self is contained within the region composed
    // of the given points.  That is, none of the lines between
    // consecutive points intersects the interior of self.  Assumes
    // that the first and last point in region are the same.
    //
    fn contained_within_region(&self, region: &[Point]) -> bool {
        // Test the line between the first and last points.
        //
        // It would be correct, but much slower, to do:
        //  !region.iter().chain(&[region[0]])
        //
        !self.interior_intersects(&region[0], region.last().unwrap()) &&
        !region.iter()
            .tuple_windows()
            .any(|(p1, p2)| self.interior_intersects(p1, p2))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    all_consuming(
        many1(
            terminated(
                separated_pair(u64, char(','), u64)
                    .map(|(x,y)| Point { x, y }),
                newline
            )
        )
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::part2_heap;

    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 4782151432);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 24);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 1450414119);
    }

    #[test]
    fn test_part2_heap_example() {
        assert_eq!(part2_heap(EXAMPLE_INPUT), 24);
    }

    #[test]
    fn test_part2_heap_full() {
        assert_eq!(part2_heap(FULL_INPUT), 1450414119);
    }
}
