use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("input/22.txt");

#[derive(Debug, Clone, Copy)]
enum Player {
    One,
    Two,
}

fn parse_player(s: &str) -> Vec<usize> {
    s.lines()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn default_input() -> (Vec<usize>, Vec<usize>) {
    let (one, two) = INPUT.split("\n\n").next_tuple().unwrap();
    (parse_player(one), parse_player(two))
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

pub fn part1((one, two): &(Vec<usize>, Vec<usize>)) -> usize {
    let (_, deck) = combat(
        VecDeque::from(one.clone()),
        VecDeque::from(two.clone()),
        false,
    );
    score(deck)
}

pub fn part2((one, two): &(Vec<usize>, Vec<usize>)) -> usize {
    let (_, deck) = combat(
        VecDeque::from(one.clone()),
        VecDeque::from(two.clone()),
        true,
    );
    score(deck)
}

#[test]
fn example1() {
    let input = (vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]);
    assert_eq!(part1(&input), 306);
    assert_eq!(part2(&input), 291);
}

#[test]
fn example2() {
    part2(&(vec![43, 19], vec![2, 29, 14]));
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 34566);
    assert_eq!(part2(&input), 31854);
}
