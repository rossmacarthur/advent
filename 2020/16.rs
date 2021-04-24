use std::collections::HashSet;
use std::ops::RangeInclusive;

use regex_macro::regex;

use itertools::Itertools;

const INPUT: &str = include_str!("input/16.txt");

fn parse_tickets(lines: &str) -> impl Iterator<Item = Ticket> + '_ {
    lines
        .lines()
        .skip(1) // header
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
}

pub fn default_input() -> Input {
    let (rules, mine, nearby) = INPUT.split("\n\n").next_tuple().unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let caps = regex!(r"(.+): ((\d+)\-(\d+)) or ((\d+)\-(\d+))")
                .captures(line)
                .unwrap();
            let left_min = caps[3].parse().unwrap();
            let left_max = caps[4].parse().unwrap();
            let right_min = caps[6].parse().unwrap();
            let right_max = caps[7].parse().unwrap();
            Rule {
                name: caps.get(1).unwrap().as_str().to_string(),
                range: (left_min..=left_max, right_min..=right_max),
            }
        })
        .collect();
    let my_ticket = parse_tickets(mine).next().unwrap();
    let nearby_tickets = parse_tickets(nearby).collect();
    (rules, my_ticket, nearby_tickets)
}

type Ticket = Vec<u64>;

type Input = (Vec<Rule>, Ticket, Vec<Ticket>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    name: String,
    range: (RangeInclusive<u64>, RangeInclusive<u64>),
}

impl Rule {
    fn matches(&self, value: u64) -> bool {
        self.range.0.contains(&value) || self.range.1.contains(&value)
    }

    fn matches_all(&self, values: &[u64]) -> bool {
        values.iter().all(|value| self.matches(*value))
    }
}

pub fn part1(input: &Input) -> u64 {
    let (rules, _, nearby_tickets) = input;
    nearby_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|&value| !rules.iter().any(|rule| rule.matches(*value)))
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    let (rules, my_ticket, nearby_tickets) = input;

    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| rules.iter().any(|rule| rule.matches(*value)))
        })
        .collect();

    let mut rule_sets: Vec<(_, HashSet<_>)> = (0..my_ticket.len())
        .map(|col| {
            let set: Vec<_> = valid_tickets.iter().map(|ticket| ticket[col]).collect();
            (
                col,
                rules
                    .iter()
                    .filter(|rule| rule.matches_all(&set))
                    .cloned()
                    .collect(),
            )
        })
        .collect();
    rule_sets.sort_by_key(|(_, rules)| rules.len());

    let mut result = 1;
    for i in 0..rule_sets.len() {
        let (col, rules) = rule_sets[i].clone();
        assert_eq!(rules.len(), 1);
        let rule = rules.into_iter().next().unwrap();
        for (_, rule_set) in rule_sets.iter_mut() {
            rule_set.remove(&rule);
        }
        if rule.name.starts_with("departure") {
            result *= my_ticket[col]
        }
    }
    result
}
