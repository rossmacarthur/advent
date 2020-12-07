use std::collections::HashSet;

use reduce::Reduce;

const INPUT: &str = include_str!("input/day06.txt");

pub fn default_input() -> Vec<Vec<HashSet<char>>> {
    INPUT
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
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
                .unwrap_or(0)
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
                .unwrap_or(0)
        })
        .sum()
}
