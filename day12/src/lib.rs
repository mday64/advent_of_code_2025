mod parsing;
use crate::parsing::parse_input;

//
// Given a set of shapes, and regions of various sizes, see if the
// given number of each shape can be arranged within the region.
//
// All of the shapes fit within a 3x3 square.  In my full input,
// regions are at most 50 in either direction.
//
pub fn part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    regions.into_iter().filter(|region| {
        let min_area: u32 = shapes.iter().zip(region.shapes.iter()).map(|(shape, count)| {
            shape.area * count
        }).sum();
        let region_area = region.width * region.length;

        if min_area <= region_area {
            eprintln!("Maybe")
        } else {
            eprintln!("No way!")
        }

        min_area <= region_area
    }).count()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

pub static FULL_INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "World");
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
