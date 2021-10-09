use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

type Orbits<'a> = HashMap<&'a str, &'a str>;

fn parse_input(input: &str) -> Orbits<'_> {
    input
        .lines()
        .map(|line| line.split(')').next_tuple().unwrap())
        .map(|(v, k)| (k, v))
        .collect()
}

fn default_input() -> Orbits<'static> {
    parse_input(include_str!("input/06.txt"))
}

fn path<'a>(orbits: &'a Orbits<'_>, to: &'a str) -> Vec<&'a str> {
    iter::successors(Some(to), move |to| orbits.get(*to).copied()).collect()
}

fn part1(orbits: &Orbits<'_>) -> usize {
    orbits.keys().map(|to| path(orbits, to).len() - 1).sum()
}

fn part2(orbits: &Orbits<'_>) -> usize {
    let you = path(orbits, "YOU");
    let san = path(orbits, "SAN");
    you.iter().take_while(|x| !san.contains(x)).count()
        + san.iter().take_while(|x| !you.contains(x)).count()
        - 2
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
    assert_eq!(part1(&input), 42);
}

#[test]
fn example2() {
    let input =
        parse_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
    assert_eq!(part2(&input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 106065);
    assert_eq!(part2(&input), 253);
}
