use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT: &str = include_str!("input/19.txt");
static EIGHT: Lazy<Rule> = Lazy::new(|| parse_rule("42 | 42 8"));
static ELEVEN: Lazy<Rule> = Lazy::new(|| parse_rule("42 31 | 42 11 31"));

#[derive(Debug, Clone)]
pub enum Rule {
    Value(char),
    And(Vec<u64>),
    Or(Vec<u64>, Vec<u64>),
}

type Input<'a> = (HashMap<u64, Rule>, Vec<&'a str>);

fn parse_rule_numbers(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_rule(rule: &str) -> Rule {
    if rule.starts_with('"') {
        Rule::Value(rule.chars().nth(1).unwrap())
    } else if rule.contains('|') {
        let (left, right) = rule.split(" | ").next_tuple().unwrap();
        Rule::Or(parse_rule_numbers(left), parse_rule_numbers(right))
    } else {
        Rule::And(parse_rule_numbers(rule))
    }
}

fn parse_input(input: &str) -> Input {
    let (rules, messages) = input.split("\n\n").next_tuple().unwrap();
    (
        rules
            .lines()
            .map(|line| {
                let (lhs, rest) = line.split(": ").next_tuple().unwrap();
                let num = lhs.parse().unwrap();
                let rule = parse_rule(rest);
                (num, rule)
            })
            .collect(),
        messages.lines().collect(),
    )
}

pub fn default_input() -> Input<'static> {
    parse_input(INPUT)
}

fn make_regex(rules: &HashMap<u64, Rule>, rule: u64, mut recursion_count: u64) -> String {
    let eight = &*EIGHT;
    let eleven = &*ELEVEN;
    let rule = match rule {
        8 if recursion_count != 0 => {
            recursion_count -= 1;
            eight
        }
        11 if recursion_count != 0 => {
            recursion_count -= 1;
            eleven
        }
        _ => &rules[&rule],
    };
    match rule {
        Rule::Value(c) => c.to_string(),
        Rule::And(sub_rules) => sub_rules
            .iter()
            .map(|&rule| make_regex(rules, rule, recursion_count))
            .collect(),
        Rule::Or(left, right) => {
            format!(
                "({}|{})",
                left.iter()
                    .map(|&rule| make_regex(rules, rule, recursion_count))
                    .collect::<String>(),
                right
                    .iter()
                    .map(|&rule| make_regex(rules, rule, recursion_count))
                    .collect::<String>()
            )
        }
    }
}

fn count_matches(input: &Input, recursion_count: u64) -> usize {
    let (rules, messages) = input;
    let re = Regex::new(&format!("^{}$", make_regex(rules, 0, recursion_count))).unwrap();
    messages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}

pub fn part1(input: &Input) -> usize {
    count_matches(input, 0)
}

pub fn part2(input: &Input) -> usize {
    count_matches(input, 5)
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
    assert_eq!(part1(&input), 2);
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
    assert_eq!(part1(&input), 3);
    assert_eq!(part2(&input), 12);
}
