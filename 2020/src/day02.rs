use std::str;

use regex_macro::regex;

const INPUT: &str = include_str!("input/day02.txt");

pub fn default_input() -> Vec<Element<'static>> {
    let re = regex!(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>.):\s(?P<password>.*)");
    re.captures_iter(INPUT)
        .map(|caps| Element {
            lower: caps["lower"].parse().unwrap(),
            upper: caps["upper"].parse().unwrap(),
            letter: caps["letter"].parse().unwrap(),
            password: caps.name("password").unwrap().as_str(),
        })
        .collect()
}

#[derive(Debug)]
pub struct Element<'i> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'i str,
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
