use day05::{ part1, part2, both, parse_input };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2() {
    part2(INPUT);
}

#[divan::bench]
fn bench_both() {
    both(INPUT);
}

#[divan::bench]
fn bench_parse() {
    parse_input(INPUT);
}
