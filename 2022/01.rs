use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(str::parse).map(Result::unwrap).collect())
        .collect()
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_str!("input/01.txt"))
}

fn part1(input: Vec<Vec<i64>>) -> i64 {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<i64>())
        .max()
        .unwrap()
}

fn part2(input: Vec<Vec<i64>>) -> i64 {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<i64>())
        .sorted_unstable()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    );
    assert_eq!(part1(input.clone()), 24000);
    assert_eq!(part2(input), 45000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 68787);
    assert_eq!(part2(input), 198041);
}
