use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .next_array()
                .map(|[l, r]| (l, r))
                .unwrap()
        })
        .collect()
}

fn default_input() -> (Vec<u64>, Vec<u64>) {
    parse_input(include_input!(2024 / 01))
}

fn part1((mut left, mut right): (Vec<u64>, Vec<u64>)) -> u64 {
    left.sort_unstable();
    right.sort_unstable();
    iter::zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2((left, right): (Vec<u64>, Vec<u64>)) -> u64 {
    let counts = right
        .into_iter()
        .fold(HashMap::with_capacity(left.len()), |mut counts, r| {
            *counts.entry(r).or_insert(0) += 1;
            counts
        });
    left.into_iter()
        .filter_map(|l| counts.get(&l).map(|&c| l * c))
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
    let input = parse_input(input);
    assert_eq!(part1(input.clone()), 11);
    assert_eq!(part2(input), 31);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2375403);
    assert_eq!(part2(input), 23082277);
}
