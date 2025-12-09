use day09::{part1, part2_heap};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");

    let result2 = part2_heap(input);
    println!("Part 2: {result2}");
}
