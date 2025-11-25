use advent::prelude::*;

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().bytes().map(|b| b - b'0').collect()
}

fn default_input() -> Vec<u8> {
    parse_input(include_input!(2017 / 01))
}

fn solve(k: usize, captcha: Vec<u8>) -> i64 {
    let n = captcha.len();
    captcha
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, d)| (d == captcha[(i + k) % n]).some(d as i64))
        .sum()
}

fn part1(captcha: Vec<u8>) -> i64 {
    solve(1, captcha)
}

fn part2(captcha: Vec<u8>) -> i64 {
    solve(captcha.len() / 2, captcha)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    for (input, exp) in [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)] {
        assert_eq!(part1(parse_input(input)), exp)
    }
}

#[test]
fn example2() {
    for (input, exp) in [
        ("1212", 6),
        ("1221", 0),
        ("123425", 4),
        ("123123", 12),
        ("12131415", 4),
    ] {
        assert_eq!(part2(parse_input(input)), exp)
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1171);
    assert_eq!(part2(input), 1024);
}
