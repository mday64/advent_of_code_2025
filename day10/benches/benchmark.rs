use day10::{ part1, part2 };

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
fn bench_part2_full_line120() {
    let input = "[.###...] (0,2,3,4,6) (0,1,3,4) (0,1,2,4,5,6) (0,2,3,5) (1,5,6) {40,182,28,34,24,186,176}\n";
    assert_eq!(part2(input), 204);
}
