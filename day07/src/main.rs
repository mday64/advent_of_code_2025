use day07::both;

fn main() {
    let input = include_str!("../input.txt");

    let (result1, result2) = both(input);
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
