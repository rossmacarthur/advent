use std::ops::ControlFlow;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (total, nums) = line.split_once(": ").unwrap();
            let total = total.parse().unwrap();
            let nums = nums
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            (total, nums)
        })
        .collect()
}

fn default_input() -> Vec<(u64, Vec<u64>)> {
    parse_input(include_input!(2024 / 07))
}

/// Check if the total can be made by the numbers in `nums`.
///
/// This is done in reverse order, checking the last number first which allows
/// us to short-circuit the recursion in many cases. We recursively check if the
/// total can be made by adding, multiplying, or concatenating the last number.
/// If so we recurse with the new total and the remaining numbers.
///
/// Assumes all numbers are non-zero.
fn check(total: u64, nums: &[u64], part2: bool) -> ControlFlow<()> {
    let l = nums.len() - 1;
    let rest = &nums[..l];
    let last = nums[l];

    if rest.is_empty() {
        if total == last {
            return ControlFlow::Break(());
        }
        return ControlFlow::Continue(());
    }

    // Add
    // ---
    // Could `total` have been been made by adding `last`?
    // This is only possible if `total` is greater than `last`.
    // If so, recurse with the new total.
    if total > last {
        check(total - last, rest, part2)?;
    }

    // Mul
    // ---
    // Could `total` have been made by multiplying by `last`?
    // This is only possible if `total` is divisible by `last`.
    // If so, recurse with the new total.
    if total.is_multiple_of(last) {
        check(total / last, rest, part2)?;
    }

    if part2 {
        // Concat
        // ------
        // Could `total` have been made by concatenating `last`?
        // This is only possible if the `total`s digits ends with `last`s
        // digits. If so, recurse with the new total which is `total` without
        // `last`s digits
        let dt = total.ilog10() + 1;
        let dl = last.ilog10() + 1;
        if dt > dl && total % 10u64.pow(dl) == last {
            check(total / 10u64.pow(dl), rest, part2)?;
        }
    }

    ControlFlow::Continue(())
}

fn part1(equations: Vec<(u64, Vec<u64>)>) -> u64 {
    equations
        .into_iter()
        .filter_map(|(total, nums)| check(total, &nums, false).is_break().some(total))
        .sum()
}

fn part2(equations: Vec<(u64, Vec<u64>)>) -> u64 {
    equations
        .into_iter()
        .filter_map(|(total, nums)| check(total, &nums, true).is_break().some(total))
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let input = parse_input(input);
    assert_eq!(part1(input.clone()), 3749);
    assert_eq!(part2(input), 11387);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 42283209483350);
    assert_eq!(part2(input), 1026766857276279);
}
