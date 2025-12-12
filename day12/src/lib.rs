mod parsing;
use crate::parsing::parse_input;
use std::iter::zip;

//
// Given a set of shapes, and regions of various sizes, see if the
// given number of each shape can be arranged within the region.
//
// All of the shapes fit within a 3x3 square.  In my full input,
// regions are at most 50 in either direction.
//
pub fn part1(input: &str) -> usize {
    let mut maybe = 0;

    let (shapes, regions) = parse_input(input);
    let num_fit = regions.into_iter().filter(|region| {
        let num_shapes: u32 = region.shapes.iter().sum();
        if num_shapes <= (region.width / 3) * (region.length / 3) {
            // eprintln!("Trivial!");
            return true;
        }

        let region_area = region.width * region.length;
        let min_area: u32 = zip(&shapes, &region.shapes)
            .map(|(shape, count)| shape.area * count )
            .sum();

        if min_area > region_area {
            // eprintln!("No way!");
            return false;
        }

        eprintln!("Maybe; needs further examination");
        maybe += 1;
        true
    }).count();

    if maybe > 0 {
        panic!("{maybe} regions need further examination")
    } else {
        num_fit
    }
}

pub static FULL_INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::{part1};
    
    static EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }
}

// A Shape is a 3x3 bitmap.  Nested Vecs is probably not the right type
// when it comes to solving the problem.  But it's enough to get the
// parsing started now.
struct Shape {
    grid: Vec<Vec<char>>,
    area: u32,
}

impl Shape {
    fn new(grid: Vec<Vec<char>>) -> Shape {
        let area = grid.iter().flatten().filter(|&&c| c == '#').count() as u32;
        Shape{grid, area}
    }
}

struct Region {
    width: u32,
    length: u32,
    shapes: Vec<u32>,   // Quantity of each shape
}
