#![allow(clippy::needless_question_mark)]

use std::str::FromStr;

use recap::Recap;
use serde::Deserialize;

const INPUT: &str = include_str!("input/day02.txt");

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>.):\s(?P<password>.*)")]
pub struct Element {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

fn parse_input(s: &str) -> Vec<Element> {
    s.lines()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}

pub fn default_input() -> Vec<Element> {
    parse_input(INPUT)
}

pub fn part1(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            let freq = e.password.chars().filter(|&c| c == e.letter).count();
            (e.lower..=e.upper).contains(&freq)
        })
        .count()
}

pub fn part2(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            (e.password.chars().nth(e.lower - 1).unwrap() == e.letter)
                ^ (e.password.chars().nth(e.upper - 1).unwrap() == e.letter)
        })
        .count()
}

#[test]
fn example() {
    let input = parse_input(
        "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc",
    );
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 1);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 383);
    assert_eq!(part2(&input), 272);
}
