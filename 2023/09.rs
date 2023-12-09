use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_input!(2023 / 09))
}

fn predict(nums: &[i64]) -> i64 {
    if nums.iter().all(|&d| d == 0) {
        return 0;
    }

    let diffs: Vec<i64> = nums
        .iter()
        .copied()
        .array_windows()
        .map(|[a, b]| b - a)
        .collect();

    return nums.last().unwrap() + predict(&diffs);
}

fn solve(dataset: Vec<Vec<i64>>) -> i64 {
    dataset.into_iter().map(|history| predict(&history)).sum()
}

fn part1(dataset: Vec<Vec<i64>>) -> i64 {
    solve(dataset)
}

fn part2(mut dataset: Vec<Vec<i64>>) -> i64 {
    for history in &mut dataset {
        history.reverse();
    }
    solve(dataset)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    );
    assert_eq!(part1(input.clone()), 114);
    assert_eq!(part2(input.clone()), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2175229206);
    assert_eq!(part2(input), 942);
}
