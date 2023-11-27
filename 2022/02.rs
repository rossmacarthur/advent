use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[u32; 2]> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|b| match b {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => unreachable!(),
                })
                .next_array()
                .unwrap()
        })
        .collect()
}

fn default_input() -> Vec<[u32; 2]> {
    parse_input(include_str!("input/02.txt"))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

const ROUNDS: &[(Move, Move, Outcome)] = &[
    (Move::Rock, Move::Rock, Outcome::Draw),
    (Move::Rock, Move::Paper, Outcome::Win),
    (Move::Rock, Move::Scissors, Outcome::Loss),
    (Move::Paper, Move::Rock, Outcome::Loss),
    (Move::Paper, Move::Paper, Outcome::Draw),
    (Move::Paper, Move::Scissors, Outcome::Win),
    (Move::Scissors, Move::Rock, Outcome::Win),
    (Move::Scissors, Move::Paper, Outcome::Loss),
    (Move::Scissors, Move::Scissors, Outcome::Draw),
];

fn part1(rounds: Vec<[u32; 2]>) -> u32 {
    rounds
        .into_iter()
        .map(|[c1, c2]| {
            ROUNDS
                .iter()
                .find_map(|&(m1, m2, oc)| {
                    let m1 = m1 as u32;
                    let m2 = m2 as u32;
                    let oc = oc as u32;
                    (m1 == c1 && m2 == c2).some(m2 + 1 + oc * 3)
                })
                .unwrap()
        })
        .sum()
}

fn part2(rounds: Vec<[u32; 2]>) -> u32 {
    rounds
        .into_iter()
        .map(|[c1, c2]| {
            ROUNDS
                .iter()
                .find_map(|&(m1, m2, oc)| {
                    let m1 = m1 as u32;
                    let m2 = m2 as u32;
                    let oc = oc as u32;
                    (m1 == c1 && oc == c2).some(m2 + 1 + oc * 3)
                })
                .unwrap()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "A Y
B X
C Z",
    );
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 12645);
    assert_eq!(part2(input), 11756);
}
