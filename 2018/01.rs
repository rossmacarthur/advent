use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/01.txt"))
}

fn part1(input: Vec<i64>) -> i64 {
    input.into_iter().sum()
}

fn part2(input: Vec<i64>) -> i64 {
    let mut cache = HashSet::from_iter([0]);
    let mut sum = 0;
    input
        .into_iter()
        .cycle()
        .find_map(|i| {
            sum += i;
            cache.replace(sum)
        })
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    for (input, result) in [("+1 +1 +1", 3), ("+1 +1 -2", 0), ("-1 -2 -3", -6)] {
        assert_eq!(part1(parse_input(input)), result)
    }
}

#[test]
fn example2() {
    for (input, result) in [
        ("+1 -1", 0),
        ("+3 +3 +4 -2 -4", 10),
        ("-6 +3 +8 +5 -6", 5),
        ("+7 +7 -2 -7 -4", 14),
    ] {
        assert_eq!(part2(parse_input(input)), result)
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 466);
    assert_eq!(part2(input), 750);
}
