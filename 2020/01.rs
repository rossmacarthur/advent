use advent::prelude::*;

const SUM: i64 = 2020;

fn default_input() -> Vec<i64> {
    include_str!("input/01.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .sorted_unstable()
        .collect()
}

fn solve(numbers: &[i64], mut i: usize, c: i64) -> Option<i64> {
    let mut j = numbers.len() - 1;
    while i < j {
        let a = numbers[i];
        let b = numbers[j];
        let sum = a + b + c;
        match sum.cmp(&SUM) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => return Some(a * b),
        }
    }
    None
}

fn part1(numbers: Vec<i64>) -> i64 {
    solve(&numbers, 0, 0).unwrap()
}

fn part2(numbers: Vec<i64>) -> i64 {
    for i in 0..numbers.len() - 2 {
        let c = numbers[i];
        if let Some(ab) = solve(&numbers, i + 1, c) {
            return ab * c;
        }
    }
    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part1(input.clone()), 514579);
    assert_eq!(part2(input), 241861950);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 567171);
    assert_eq!(part2(input), 212428694);
}
