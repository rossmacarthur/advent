use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(from_snafu).collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2022 / 25))
}

fn from_snafu(s: &str) -> i64 {
    let mut n = 0;
    for c in s.chars() {
        let d = ['0', '1', '2', '=', '-']
            .iter()
            .position(|&d| d == c)
            .unwrap() as i64;
        n = n * 5 + d;
        if d > 2 {
            n -= 5;
        }
    }
    n
}

fn to_snafu(mut n: i64) -> String {
    let mut s = String::new();
    while n != 0 {
        let d = n.rem_euclid(5) as usize;
        n /= 5;
        s.push(['0', '1', '2', '=', '-'][d]);
        if d > 2 {
            n += 1;
        }
    }
    s.chars().rev().collect()
}

fn part1(input: Vec<i64>) -> String {
    to_snafu(input.into_iter().sum())
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "2=-0=01----22-0-1-10");
}

#[test]
fn to_and_from() {
    for (dec, snafu) in [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ] {
        assert_eq!(to_snafu(dec), snafu);
        assert_eq!(from_snafu(snafu), dec);
    }
}
