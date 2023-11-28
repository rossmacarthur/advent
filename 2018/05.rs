use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 05).trim()
}

fn reacts(a: u8, b: u8) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a != b
}

fn react(polymer: impl Iterator<Item = u8>) -> usize {
    let mut stack = Vec::new();
    for a in polymer {
        match stack.last().copied() {
            Some(b) if reacts(a, b) => {
                stack.pop();
            }
            _ => {
                stack.push(a);
            }
        }
    }
    stack.len()
}

fn part1(suit: &str) -> usize {
    react(suit.bytes())
}

fn part2(suit: &str) -> usize {
    (b'a'..=b'z')
        .map(|a| {
            let polymer = suit
                .bytes()
                .filter(|&b| b != a && b != a.to_ascii_uppercase());
            react(polymer)
        })
        .min()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "dabAcCaCBAcCcaDA";
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 10564);
    assert_eq!(part2(input), 6336);
}
