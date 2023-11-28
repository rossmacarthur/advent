use advent::prelude::*;

fn parse_input(input: &str) -> (usize, usize) {
    let caps = regex!(r"(\d+) players; last marble is worth (\d+) points")
        .captures(input)
        .unwrap();
    let players = caps[1].parse().unwrap();
    let marbles = caps[2].parse().unwrap();
    (players, marbles)
}

fn default_input() -> (usize, usize) {
    parse_input(include_input!(2018 / 09))
}

fn solve(players: usize, marbles: usize) -> usize {
    let mut scores = HashMap::new();
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_back(0);
    for (m, p) in (1..=marbles).zip((0..players).cycle()) {
        if m % 23 == 0 {
            circle.rotate_right(8);
            *scores.entry(p).or_default() += m + circle.pop_front().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(m);
        }
    }
    scores.values().copied().max().unwrap()
}

fn part1((players, marbles): (usize, usize)) -> usize {
    solve(players, marbles)
}

fn part2((players, marbles): (usize, usize)) -> usize {
    solve(players, 100 * marbles)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    for (input, exp) in [
        ("9 players; last marble is worth 25 points", 32),
        ("10 players; last marble is worth 1618 points", 8317),
        ("13 players; last marble is worth 7999 points", 146373),
        ("17 players; last marble is worth 1104 points", 2764),
        ("21 players; last marble is worth 6111 points", 54718),
        ("30 players; last marble is worth 5807 points", 37305),
    ] {
        let input = parse_input(input);
        assert_eq!(part1(input), exp);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 429943);
    assert_eq!(part2(input), 3615691746);
}
