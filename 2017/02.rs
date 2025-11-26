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
    parse_input(include_input!(2017 / 02))
}

fn part1(sheet: Vec<Vec<i64>>) -> i64 {
    sheet
        .into_iter()
        .map(|row| row.iter().min_max().map(|(a, b)| b - a).unwrap())
        .sum()
}

fn part2(sheet: Vec<Vec<i64>>) -> i64 {
    sheet
        .into_iter()
        .map(|row| {
            row.into_iter()
                .sorted()
                .array_combinations()
                .find_map(|[a, b]| if b % a == 0 { Some(b / a) } else { None })
                .unwrap()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input("5 1 9 5\n7 5 3\n2 4 6 8");
    assert_eq!(part1(input.clone()), 18);
}

#[test]
fn example2() {
    let input = parse_input("5 9 2 8\n9 4 7 3\n3 8 6 5");
    assert_eq!(part2(input.clone()), 9);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 37923);
    assert_eq!(part2(input), 263);
}
