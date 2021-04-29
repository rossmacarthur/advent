use std::cmp::Ordering::*;

const SUM: u64 = 2020;

fn default_input() -> Vec<u64> {
    include_str!("input/01.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
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

fn part1(input: &[u64]) -> u64 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    solve(&numbers, 0, 0).unwrap()
}

fn part2(input: &[u64]) -> u64 {
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

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part1(&input), 514579);
    assert_eq!(part2(&input), 241861950);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 567171);
    assert_eq!(part2(&input), 212428694);
}
