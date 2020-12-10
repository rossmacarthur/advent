use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day06.txt");

type Orbits<'a> = HashMap<&'a str, &'a str>;

pub fn default_input() -> Orbits<'static> {
    INPUT
        .lines()
        .map(|line| line.split(')').next_tuple().unwrap())
        .map(|(v, k)| (k, v))
        .collect()
}

fn path<'a>(orbits: &'a Orbits, to: &'a str) -> Vec<&'a str> {
    iter::successors(Some(to), move |to| orbits.get(*to).copied()).collect()
}

pub fn part1(orbits: &Orbits) -> usize {
    orbits.keys().map(|to| path(orbits, to).len() - 1).sum()
}

pub fn part2(orbits: &Orbits) -> usize {
    let you = path(orbits, "YOU");
    let san = path(orbits, "SAN");
    you.iter().take_while(|x| !san.contains(x)).count()
        + san.iter().take_while(|x| !you.contains(x)).count()
        - 2
}
