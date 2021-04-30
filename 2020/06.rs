use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn default_input() -> Vec<Vec<HashSet<char>>> {
    parse_input(include_str!("input/06.txt"))
}

fn part1(input: &[Vec<HashSet<char>>]) -> usize {
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

fn part2(input: &[Vec<HashSet<char>>]) -> usize {
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

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
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
