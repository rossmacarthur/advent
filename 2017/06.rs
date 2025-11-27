use std::collections::hash_map::Entry;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2017 / 06))
}

fn solve(mut banks: Vec<usize>, part2: bool) -> usize {
    let len = banks.len();
    let mut cycles: usize = 0;
    let mut states = HashMap::new();

    loop {
        match states.entry(banks.clone().into_boxed_slice()) {
            Entry::Occupied(entry) => break if part2 { cycles - *entry.get() } else { cycles },
            Entry::Vacant(entry) => entry.insert(cycles),
        };
        cycles += 1;

        let (i, blocks) = banks
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(i, b)| (b, Reverse(i)))
            .unwrap();

        banks[i] = 0;
        for j in 0..blocks {
            banks[(i + j + 1) % len] += 1;
        }
    }
}

fn part1(banks: Vec<usize>) -> usize {
    solve(banks, false)
}

fn part2(banks: Vec<usize>) -> usize {
    solve(banks, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("0  2  7  0");
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7864);
    assert_eq!(part2(input), 1695);
}
