use advent::prelude::*;

type Ticket = Vec<i64>;

fn parse_tickets(input: &str) -> impl Iterator<Item = Ticket> + '_ {
    input
        .lines()
        .skip(1) // header
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
}

fn parse_input(input: &str) -> (Vec<Rule<'_>>, Ticket, Vec<Ticket>) {
    let [rules, your, nearby] = input.split("\n\n").next_array().unwrap();
    let rules =
        regex!(r"(?P<name>.+): (?P<lmin>\d+)\-(?P<lmax>\d+) or (?P<rmin>\d+)\-(?P<rmax>\d+)")
            .captures_iter(rules)
            .map(|caps| Rule {
                name: caps.name("name").unwrap().as_str(),
                lmin: caps["lmin"].parse().unwrap(),
                lmax: caps["lmax"].parse().unwrap(),
                rmin: caps["rmin"].parse().unwrap(),
                rmax: caps["rmax"].parse().unwrap(),
            })
            .collect();
    let your = parse_tickets(your).next().unwrap();
    let nearby = parse_tickets(nearby).collect();
    (rules, your, nearby)
}

fn default_input() -> (Vec<Rule<'static>>, Ticket, Vec<Ticket>) {
    parse_input(include_str!("input/16.txt"))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<'a> {
    name: &'a str,
    lmin: i64,
    lmax: i64,
    rmin: i64,
    rmax: i64,
}

impl Rule<'_> {
    fn matches(&self, v: &i64) -> bool {
        (self.lmin..=self.lmax).contains(v) || (self.rmin..=self.rmax).contains(v)
    }
}

fn part1((rules, _, nearby): (Vec<Rule<'_>>, Ticket, Vec<Ticket>)) -> i64 {
    nearby
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|v| !rules.iter().any(|rule| rule.matches(v)))
        })
        .sum()
}

fn part2((rules, your, nearby): (Vec<Rule<'_>>, Ticket, Vec<Ticket>)) -> i64 {
    // First we find all the tickets that match any rule correctly.
    let valid: Vec<_> = nearby
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|v| rules.iter().any(|rule| rule.matches(v)))
        })
        .collect();

    // Now construct a map of column to set of rules that might apply.
    let mut rule_sets: Vec<_> = (0..your.len())
        .map(|col| {
            // The column values for the valid tickets.
            let set: Vec<_> = valid.iter().map(|ticket| ticket[col]).collect();
            // The set of rules that match all the column values.
            let rules: HashSet<_> = rules
                .iter()
                .filter(|rule| set.iter().all(|v| rule.matches(v)))
                .collect();
            (col, rules)
        })
        .sorted_unstable_by_key(|(_, rules)| Reverse(rules.len()))
        .collect();

    let mut result = 1;
    while let Some((col, rules)) = rule_sets.pop() {
        assert_eq!(rules.len(), 1);
        let rule = rules.into_iter().next().unwrap();
        for (_, rule_set) in rule_sets.iter_mut() {
            rule_set.remove(&rule);
        }
        if rule.name.starts_with("departure") {
            result *= your[col]
        }
    }
    result
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
    );
    assert_eq!(part1(input.clone()), 71);
    assert_eq!(part2(input), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 29019);
    assert_eq!(part2(input), 517827547723);
}
