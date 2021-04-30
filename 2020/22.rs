use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Input = (VecDeque<usize>, VecDeque<usize>);

fn parse_player(s: &str) -> VecDeque<usize> {
    s.lines()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Input {
    let (one, two) = include_str!("input/22.txt")
        .split("\n\n")
        .next_tuple()
        .unwrap();
    (parse_player(one), parse_player(two))
}

#[derive(Debug, Clone, Copy)]
enum Player {
    One,
    Two,
}

fn score(deck: VecDeque<usize>) -> usize {
    deck.into_iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (card * (i + 1)))
        .sum()
}

fn combat(
    mut deck1: VecDeque<usize>,
    mut deck2: VecDeque<usize>,
    recurse: bool,
) -> (Player, VecDeque<usize>) {
    let mut previous = HashSet::new();
    loop {
        if recurse {
            if previous.contains(&(deck1.clone(), deck2.clone())) {
                return (Player::One, deck1);
            } else {
                previous.insert((deck1.clone(), deck2.clone()));
            }
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let winner = if recurse && card1 <= deck1.len() && card2 <= deck2.len() {
            let (winner, _) = combat(
                deck1.iter().copied().take(card1).collect(),
                deck2.iter().copied().take(card2).collect(),
                recurse,
            );
            winner
        } else if card1 > card2 {
            Player::One
        } else if card1 < card2 {
            Player::Two
        } else {
            unreachable!()
        };

        match winner {
            Player::One => {
                deck1.push_back(card1);
                deck1.push_back(card2);
                if deck2.is_empty() {
                    break (Player::One, deck1);
                }
            }
            Player::Two => {
                deck2.push_back(card2);
                deck2.push_back(card1);
                if deck1.is_empty() {
                    break (Player::Two, deck2);
                }
            }
        }
    }
}

fn part1((one, two): &Input) -> usize {
    let (_, deck) = combat(one.clone(), two.clone(), false);
    score(deck)
}

fn part2((one, two): &Input) -> usize {
    let (_, deck) = combat(one.clone(), two.clone(), true);
    score(deck)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[cfg(test)]
macro_rules! deq {
    ($($i:expr),*) => (VecDeque::from(vec![$($i),*]));
}

#[test]
fn example1() {
    let input = (deq![9, 2, 6, 3, 1], deq![5, 8, 4, 7, 10]);
    assert_eq!(part1(&input), 306);
    assert_eq!(part2(&input), 291);
}

#[test]
fn example2() {
    part2(&(deq![43, 19], deq![2, 29, 14]));
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 34566);
    assert_eq!(part2(&input), 31854);
}
