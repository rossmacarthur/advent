use advent::prelude::*;

fn parse_hand(h: &str) -> Hand {
    h.chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as u8,
        })
        .next_array()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<(Hand, i64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (parse_hand(hand), bid.parse().unwrap())
        })
        .collect()
}

fn default_input() -> Vec<(Hand, i64)> {
    parse_input(include_input!(2023 / 07))
}

/// Represents a hand of cards with each card represented by a number.
///
/// 0 represents a joker
/// 1 - 10 represent the cards 1 - 10
/// 11 - 14 represent the cards J, Q, K, A
type Hand = [u8; 5];

/// Replaces all occurrences of a card in a hand with another.
fn replace(mut hand: Hand, c1: u8, c2: u8) -> Hand {
    for c in &mut hand {
        if *c == c1 {
            *c = c2;
        }
    }
    hand
}

/// Returns a key for a hand that allows hands to be compared.
fn key(hand: Hand) -> [u8; 5] {
    let mut k = [0; 5];
    for (i, card) in hand.iter().enumerate() {
        k[i] = hand.iter().filter(|&c| c == card).count() as u8;
    }
    k.sort_unstable();
    k.reverse();
    k
}

fn solve<F>(hands: Vec<(Hand, i64)>, key_fn: F) -> i64
where
    F: Fn(Hand) -> [u8; 5],
{
    hands
        .into_iter()
        .sorted_by_cached_key(|&(hand, _)| (key_fn(hand), hand))
        .enumerate()
        .map(|(r, (_, bid))| (r as i64 + 1) * bid)
        .sum()
}

fn part1(hands: Vec<(Hand, i64)>) -> i64 {
    solve(hands, key)
}

fn part2(hands: Vec<(Hand, i64)>) -> i64 {
    // Replace all jacks with jokers
    let hands = hands
        .into_iter()
        .map(|(hand, bid)| (replace(hand, 11, 0), bid))
        .collect();

    solve(hands, |hand| {
        hand.iter()
            .map(|&card| key(replace(hand, 0, card)))
            .max()
            .unwrap()
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    );
    assert_eq!(part1(input.clone()), 6440);
    assert_eq!(part2(input), 5905);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 246409899);
    assert_eq!(part2(input), 244848487);
}
