use std::ops::Deref;

use advent::prelude::*;

fn parse_seq(s: &str) -> Seq {
    s.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .fold(Seq::default(), |mut acc, id| {
            acc.arr[acc.len] = id;
            acc.len += 1;
            acc
        })
}

fn parse_rule(rule: &str) -> Rule {
    if rule.starts_with('"') {
        let c = rule.chars().nth(1).unwrap();
        Rule::Exact(c)
    } else if rule.contains('|') {
        let (left, right) = rule.split_once(" | ").unwrap();
        Rule::Or(parse_seq(left), parse_seq(right))
    } else {
        Rule::And(parse_seq(rule))
    }
}

fn parse_input(input: &str) -> (HashMap<u8, Rule>, Vec<&str>) {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (lhs, rest) = line.split_once(": ").unwrap();
            let num = lhs.parse().unwrap();
            let rule = parse_rule(rest);
            (num, rule)
        })
        .collect();
    (rules, messages.lines().collect())
}

fn default_input() -> (HashMap<u8, Rule>, Vec<&'static str>) {
    parse_input(include_str!("input/19.txt"))
}

#[derive(Clone)]
enum Rule {
    Exact(char),
    And(Seq),
    Or(Seq, Seq),
}

#[derive(Clone, Default)]
struct Seq {
    arr: [u8; 3],
    len: usize,
}

impl Deref for Seq {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.arr[..self.len]
    }
}

/// Recursively attempts to match the given rules to the given message.
fn matches(rules: &HashMap<u8, Rule>, msg: &str, seq: Vec<u8>) -> bool {
    // This is our base case. If the message is fully parsed and there are no
    // more rules to match then return true.
    if msg.is_empty() || seq.is_empty() {
        return msg.is_empty() && seq.is_empty();
    }

    // Get the first rule to test.
    let (rule, rest) = seq.split_first().unwrap();

    // Tests if the message matches all the provided rules.
    let all = |seq: &Seq| {
        let ids = seq.iter().chain(rest).copied().collect();
        matches(rules, msg, ids)
    };

    match &rules[rule] {
        Rule::Exact(c) => msg
            .strip_prefix(*c)
            .map(|part| matches(rules, part, rest.to_vec()))
            .unwrap_or(false),
        Rule::And(seq) => all(seq),
        Rule::Or(left, right) => all(left) || all(right),
    }
}

fn count(rules: &HashMap<u8, Rule>, messages: &[&str]) -> usize {
    messages
        .iter()
        .filter(|msg| matches(rules, msg, vec![0]))
        .count()
}

fn part1((rules, messages): (HashMap<u8, Rule>, Vec<&'static str>)) -> usize {
    count(&rules, &messages)
}

fn part2((mut rules, messages): (HashMap<u8, Rule>, Vec<&str>)) -> usize {
    rules.insert(8, parse_rule("42 | 42 8"));
    rules.insert(11, parse_rule("42 31 | 42 11 31"));
    count(&rules, &messages)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#,
    );
    assert_eq!(part1(input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#,
    );
    assert_eq!(part1(input.clone()), 3);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 178);
    assert_eq!(part2(input), 346);
}
