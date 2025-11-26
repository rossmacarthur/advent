use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.as_bytes().to_vec())
                .collect()
        })
        .collect()
}

fn default_input() -> Vec<Vec<Vec<u8>>> {
    parse_input(include_input!(2017 / 04))
}

fn part1(input: Vec<Vec<Vec<u8>>>) -> usize {
    input
        .into_iter()
        .filter(|phrase| HashSet::from_iter(phrase).len() == phrase.len())
        .count()
}

fn part2(mut input: Vec<Vec<Vec<u8>>>) -> usize {
    for phrase in &mut input {
        for word in phrase {
            word.sort_unstable();
        }
    }
    part1(input)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa",
    );
    assert_eq!(part1(input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio
",
    );
    assert_eq!(part2(input), 3)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 477);
    assert_eq!(part2(input), 167);
}
