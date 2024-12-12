use std::collections::HashMap;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<u64> {
    parse_input(include_input!(2024 / 11))
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    stones
        .into_iter()
        .map(|(stone, count)| {
            if stone == 0 {
                return Either::Left([(1, count)]);
            }

            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let half = 10_u64.pow(digits / 2);
                let left = stone / half;
                let right = stone % half;
                return Either::Right([(left, count), (right, count)]);
            }

            Either::Left([(stone * 2024, count)])
        })
        .flat_map(|either| either.into_iter())
        .fold(HashMap::new(), |mut acc, (stone, count)| {
            *acc.entry(stone).or_insert(0) += count;
            acc
        })
}

fn solve(stones: Vec<u64>, blinks: usize) -> u64 {
    let mut stones = stones.into_iter().fold(HashMap::new(), |mut acc, stone| {
        *acc.entry(stone).or_insert(0) += 1;
        acc
    });
    for _ in 0..blinks {
        stones = blink(stones);
    }
    stones.values().sum()
}

fn part1(stones: Vec<u64>) -> u64 {
    solve(stones, 25)
}

fn part2(stones: Vec<u64>) -> u64 {
    solve(stones, 75)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("125 17");
    assert_eq!(part1(input.clone()), 55312);
    assert_eq!(part2(input), 65601038650482);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 228668);
    assert_eq!(part2(input), 270673834779359);
}
