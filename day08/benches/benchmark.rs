use day08::{ part1, part2, both };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT, 1000);
}

#[divan::bench]
fn bench_part2() {
    part2(INPUT);
}

#[divan::bench]
fn bench_both() {
    both(INPUT, 1000);
}
