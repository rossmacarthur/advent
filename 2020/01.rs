use std::cmp::Ordering::*;

const INPUT: &str = include_str!("input/01.txt");

const SUM: u64 = 2020;

pub fn default_input() -> Vec<u64> {
    INPUT.lines().map(str::parse).map(Result::unwrap).collect()
}

fn solve(numbers: &[u64], mut left: usize, c: u64) -> Option<u64> {
    let mut right = numbers.len() - 1;
    while left < right {
        let a = numbers[left];
        let b = numbers[right];
        let sum = a + b + c;
        match sum.cmp(&SUM) {
            Less => left += 1,
            Greater => right -= 1,
            Equal => return Some(a * b),
        }
    }
    None
}

pub fn part1(input: &[u64]) -> u64 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    solve(&numbers, 0, 0).unwrap()
}

pub fn part2(input: &[u64]) -> u64 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    for i in 0..numbers.len() - 2 {
        let c = numbers[i];
        if let Some(a_x_b) = solve(&numbers, i + 1, c) {
            return a_x_b * c;
        }
    }
    unreachable!()
}

#[test]
fn example() {
    let input = [1721, 979, 366, 299, 675, 1456];
    assert_eq!(part1(&input), 514579);
    assert_eq!(part2(&input), 241861950);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 567171);
    assert_eq!(part2(&input), 212428694);
}