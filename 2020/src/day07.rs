use std::collections::{HashMap, HashSet};

use regex_macro::regex;

const INPUT: &str = include_str!("input/day07.txt");
const COLOR: &str = "shiny gold";

type Rules<'a> = HashMap<&'a str, Vec<(&'a str, u64)>>;

pub fn default_input() -> Rules<'static> {
    INPUT
        .lines()
        .map(|rule| {
            let caps = regex!(r"^(\w+ \w+) bags contain (.*)\.$")
                .captures(rule)
                .unwrap();
            let color = caps.get(1).unwrap().as_str();
            let contents = regex!(r"(\d+) (\w+ \w+) bags?")
                .captures_iter(caps.get(2).unwrap().as_str())
                .map(|captures| {
                    let count = captures.get(1).unwrap().as_str().parse().unwrap();
                    let color = &captures.get(2).unwrap().as_str();
                    (*color, count)
                })
                .collect();
            (color, contents)
        })
        .collect()
}

fn find<'a>(
    reversed: &'a HashMap<&'a str, Vec<&'a str>>,
    color: &'a str,
    found: &mut HashSet<&'a str>,
) {
    if let Some(colors) = reversed.get(color) {
        for color in colors {
            found.insert(color);
            find(reversed, color, found);
        }
    }
}

pub fn part1(rules: &Rules) -> usize {
    let mut reversed = HashMap::new();
    for (color, contains) in rules {
        for (in_color, _) in contains {
            reversed
                .entry(*in_color)
                .or_insert_with(Vec::new)
                .push(*color);
        }
    }
    let mut found = HashSet::new();
    find(&reversed, COLOR, &mut found);
    found.len()
}

fn count(rules: &Rules, color: &str) -> u64 {
    rules[color]
        .iter()
        .map(|(color, i)| i * (1 + count(rules, color)))
        .sum()
}

pub fn part2(rules: &Rules) -> u64 {
    count(rules, COLOR)
}
