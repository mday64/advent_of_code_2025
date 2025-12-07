use day07::{ part1, part2, both, both_array };

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
fn bench_both_array() {
    both_array(INPUT);
}
