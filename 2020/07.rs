use std::collections::{HashMap, HashSet};

use regex_macro::regex;

type Rules<'a> = HashMap<&'a str, Vec<(&'a str, u64)>>;

const COLOR: &str = "shiny gold";

fn parse_input(input: &str) -> Rules {
    input
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

fn default_input() -> Rules<'static> {
    parse_input(include_str!("input/07.txt"))
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

fn count(rules: &Rules, color: &str) -> u64 {
    rules[color]
        .iter()
        .map(|(color, i)| i * (1 + count(rules, color)))
        .sum()
}

fn part1(rules: &Rules) -> usize {
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

fn part2(rules: &Rules) -> u64 {
    count(rules, COLOR)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#,
    );
    assert_eq!(part1(&input), 4);
    assert_eq!(part2(&input), 32);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#,
    );
    assert_eq!(part1(&input), 0);
    assert_eq!(part2(&input), 126);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 101);
    assert_eq!(part2(&input), 108636);
}
