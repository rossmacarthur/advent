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
    parse_input(include_input!(2024 / 02))
}

fn is_safe(report: impl Iterator<Item = i64>) -> bool {
    report
        .array_windows()
        .try_fold(Ordering::Equal, |prev, [a, b]| {
            match (prev, a.cmp(&b)) {
                (_, Ordering::Equal) => None,
                (Ordering::Less, Ordering::Greater) => None,
                (Ordering::Greater, Ordering::Less) => None,
                (Ordering::Equal, ord) => Some(ord),
                (_, ord) => Some(ord),
            }
            .filter(|_| (1..=3).contains(&a.abs_diff(b)))
        })
        .is_some()
}

fn part1(reports: Vec<Vec<i64>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report.iter().copied()))
        .count()
}

fn part2(reports: Vec<Vec<i64>>) -> usize {
    reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let before = report[0..i].iter().copied();
                let after = report[i + 1..].iter().copied();
                is_safe(before.chain(after))
            })
        })
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 463);
    assert_eq!(part2(input), 514);
}
