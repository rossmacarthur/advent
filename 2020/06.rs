use std::collections::HashSet;

const INPUT: &str = include_str!("input/06.txt");

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

pub fn default_input() -> Vec<Vec<HashSet<char>>> {
    parse_input(INPUT)
}

pub fn part1(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .cloned()
                .reduce(|acc, person| acc.union(&person).cloned().collect())
                .map(|set| set.len())
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .cloned()
                .reduce(|acc, person| acc.intersection(&person).cloned().collect())
                .map(|set| set.len())
                .unwrap()
        })
        .sum()
}

#[test]
fn example() {
    let input = parse_input(
        r#"abc

a
b
c

ab
ac

a
a
a
a

b"#,
    );
    assert_eq!(part1(&input), 11);
    assert_eq!(part2(&input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 6587);
    assert_eq!(part2(&input), 3235);
}
