use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[i64; 4]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|s| s.split('-'))
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap()
        })
        .collect()
}

fn default_input() -> Vec<[i64; 4]> {
    parse_input(include_input!(2022 / 04))
}

fn part1(pairs: Vec<[i64; 4]>) -> usize {
    pairs
        .into_iter()
        .filter(|&[a, b, m, n]| (a >= m && b <= n) || (m >= a && n <= b))
        .count()
}

fn part2(pairs: Vec<[i64; 4]>) -> usize {
    pairs
        .into_iter()
        .filter(|&[a, b, m, n]| (a <= m && b >= m) || (m <= a && n >= a))
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 576);
    assert_eq!(part2(input), 905);
}
