use std::str;

use regex_macro::regex;

const INPUT: &str = include_str!("day02.txt");

pub fn default_input() -> Vec<Element> {
    Element::parse_multiple(INPUT)
}

#[derive(Debug)]
pub struct Element {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl Element {
    fn parse_multiple(input: &str) -> Vec<Self> {
        let re = regex!(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>.):\s(?P<password>.*)");
        re.captures_iter(input)
            .map(|caps| {
                let lower = caps["lower"].parse().unwrap();
                let upper = caps["upper"].parse().unwrap();
                let letter = caps["letter"].parse().unwrap();
                let password = caps["password"].to_string();
                Self {
                    lower,
                    upper,
                    letter,
                    password,
                }
            })
            .collect()
    }
}

pub fn valid_with_count_policy(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            let freq = e.password.chars().filter(|&c| c == e.letter).count();
            (e.lower..=e.upper).contains(&freq)
        })
        .count()
}

pub fn valid_with_position_policy(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            (e.password.chars().nth(e.lower - 1).unwrap() == e.letter)
                ^ (e.password.chars().nth(e.upper - 1).unwrap() == e.letter)
        })
        .count()
}
