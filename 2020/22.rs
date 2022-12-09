use advent::prelude::*;

fn parse_player(s: &str) -> VecDeque<usize> {
    s.lines()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> [VecDeque<usize>; 2] {
    include_str!("input/22.txt")
        .split("\n\n")
        .map(parse_player)
        .next_array()
        .unwrap()
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

fn combat([mut deck1, mut deck2]: [VecDeque<usize>; 2]) -> VecDeque<usize> {
    loop {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let winner = match card1.cmp(&card2) {
            Ordering::Greater => Player::One,
            Ordering::Less => Player::Two,
            Ordering::Equal => unreachable!(),
        };

        match winner {
            Player::One => {
                deck1.push_back(card1);
                deck1.push_back(card2);
                if deck2.is_empty() {
                    return deck1;
                }
            }
            Player::Two => {
                deck2.push_back(card2);
                deck2.push_back(card1);
                if deck1.is_empty() {
                    return deck2;
                }
            }
        }
    }
}

fn recursive_combat(mut decks: [VecDeque<usize>; 2]) -> (Player, VecDeque<usize>) {
    let mut previous = HashSet::new();
    loop {
        if previous.contains(&decks) {
            return (Player::One, decks[0].clone());
        }
        previous.insert(decks.clone());

        let [deck1, deck2] = &mut decks;
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            let new1 = deck1.iter().copied().take(card1).collect();
            let new2 = deck2.iter().copied().take(card2).collect();
            let (winner, _) = recursive_combat([new1, new2]);
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
                    return (Player::One, mem::take(deck1));
                }
            }
            Player::Two => {
                deck2.push_back(card2);
                deck2.push_back(card1);
                if deck1.is_empty() {
                    return (Player::Two, mem::take(deck2));
                }
            }
        }
    }
}

fn part1(decks: [VecDeque<usize>; 2]) -> usize {
    let deck = combat(decks);
    score(deck)
}

fn part2(decks: [VecDeque<usize>; 2]) -> usize {
    let (_, deck) = recursive_combat(decks);
    score(deck)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = [
        VecDeque::from([9, 2, 6, 3, 1]),
        VecDeque::from([5, 8, 4, 7, 10]),
    ];
    assert_eq!(part1(input.clone()), 306);
    assert_eq!(part2(input), 291);
}

#[test]
fn example2() {
    part2([VecDeque::from([43, 19]), VecDeque::from([2, 29, 14])]);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 34566);
    assert_eq!(part2(input), 31854);
}
