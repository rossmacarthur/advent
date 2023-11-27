use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn default_input() -> Vec<Vec<HashSet<char>>> {
    parse_input(include_str!("input/06.txt"))
}

fn part1(input: Vec<Vec<HashSet<char>>>) -> usize {
    input
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .reduce(|acc, person| acc.union(&person).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn part2(input: Vec<Vec<HashSet<char>>>) -> usize {
    input
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .reduce(|acc, person| acc.intersection(&person).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "abc

a
b
c

ab
ac

a
a
a
a

b",
    );
    assert_eq!(part1(input.clone()), 11);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6587);
    assert_eq!(part2(input), 3235);
}
