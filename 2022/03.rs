use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.bytes().map(priority).collect())
        .collect()
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_str!("input/03.txt"))
}

fn priority(item: u8) -> i64 {
    let p = match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        i => panic!("unknown item `{}`", i),
    };
    p as i64
}

fn part1(rucksacks: Vec<Vec<i64>>) -> i64 {
    rucksacks
        .into_iter()
        .map(|rucksack| {
            let m = rucksack.len() / 2;
            let a = HashSet::from_iter(&rucksack[..m]);
            let b = HashSet::from_iter(&rucksack[m..]);
            a.intersection(&b).copied().sum::<i64>()
        })
        .sum()
}

fn part2(rucksacks: Vec<Vec<i64>>) -> i64 {
    rucksacks
        .into_iter()
        .map(HashSet::from_iter)
        .array_chunked()
        .map(|[a, b, c]| {
            let tmp = a.intersection(&b).copied().collect();
            c.intersection(&tmp).copied().sum::<i64>()
        })
        .sum()
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
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    assert_eq!(part1(input.clone()), 157);
    assert_eq!(part2(input), 70);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7850);
    assert_eq!(part2(input), 2581);
}
