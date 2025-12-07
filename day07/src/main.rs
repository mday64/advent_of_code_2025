use day07::both_array;

fn main() {
    let input = include_str!("../input.txt");

    let (result1, result2) = both_array(input);
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
