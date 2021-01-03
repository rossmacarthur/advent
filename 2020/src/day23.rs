use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

fn parse_input(s: &str) -> Vec<u64> {
    s.chars().map(|c| c.to_digit(10).unwrap().into()).collect()
}

pub fn default_input() -> Vec<u64> {
    parse_input("716892543")
}

pub fn play(cups: Vec<u64>, moves: usize, limit: Option<u64>) -> HashMap<u64, u64> {
    let max = cups.iter().max().copied().unwrap();
    let limit = limit.unwrap_or(max);
    let mut curr = cups[0];
    let mut circle: HashMap<_, _> = cups
        .into_iter()
        .chain(max + 1..=limit)
        .chain(iter::once(curr))
        .tuple_windows()
        .collect();
    for _ in 0..moves {
        let cup_a = circle[&curr];
        let cup_b = circle[&cup_a];
        let cup_c = circle[&cup_b];
        let mut dest = curr - 1;
        while dest == 0 || [cup_a, cup_b, cup_c].contains(&dest) {
            if dest == 0 {
                dest = limit;
            } else {
                dest -= 1;
            }
        }
        circle.insert(curr, circle[&cup_c]);
        circle.insert(cup_c, circle[&dest]);
        circle.insert(dest, cup_a);
        curr = circle[&curr];
    }
    circle
}

pub fn part1(cups: &[u64]) -> String {
    let circle = play(cups.to_vec(), 100, None);
    iter::successors(Some(1), |cup| Some(circle[&cup]))
        .take(cups.len())
        .skip(1)
        .map(|cup| cup.to_string())
        .collect()
}

pub fn part2(cups: &[u64]) -> u64 {
    let circle = play(cups.to_vec(), 10_000_000, Some(1_000_000));
    let cup = circle[&1];
    cup * circle[&cup]
}

#[test]
fn example() {
    let input = parse_input("389125467");
    assert_eq!(part1(&input), "67384529");
    assert_eq!(part2(&input), 149245887792);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), "49725386");
    assert_eq!(part2(&input), 538935646702);
}
