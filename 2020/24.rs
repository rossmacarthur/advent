use std::collections::HashSet;

use regex_macro::regex;

use vectrix::Vector2;

const INPUT: &str = include_str!("input/24.txt");

type Vector = Vector2<i64>;

fn parse_input(s: &str) -> Vec<Vec<Vector>> {
    s.lines()
        .map(|line| {
            regex!(r"(e|se|sw|w|nw|ne)")
                .find_iter(line)
                .map(|cap| match cap.as_str() {
                    "e" => [2, 0],
                    "se" => [1, -1],
                    "sw" => [-1, -1],
                    "w" => [-2, 0],
                    "nw" => [-1, 1],
                    "ne" => [1, 1],
                    d => panic!("unexpected direction `{}`", d),
                })
                .map(Vector::from)
                .collect()
        })
        .collect()
}

pub fn default_input() -> Vec<Vec<Vector>> {
    parse_input(INPUT)
}

fn neighbours(center: Vector) -> Vec<Vector> {
    [[2, 0], [1, -1], [-1, -1], [-2, 0], [-1, 1], [1, 1]]
        .iter()
        .copied()
        .map(Vector::from)
        .map(|direction| center + direction)
        .collect()
}

fn black_neighbours(state: &HashSet<Vector>, center: Vector) -> usize {
    neighbours(center)
        .into_iter()
        .filter_map(|vector| state.get(&vector))
        .count()
}

fn initial_state(input: &[Vec<Vector>]) -> HashSet<Vector> {
    let mut state = HashSet::new();
    for path in input {
        let location = path.iter().copied().sum();
        if state.contains(&location) {
            state.remove(&location);
        } else {
            state.insert(location);
        }
    }
    state
}

fn next_state(state: HashSet<Vector>) -> HashSet<Vector> {
    state
        .iter()
        .copied()
        .flat_map(neighbours)
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&vector| {
            let black = black_neighbours(&state, vector);
            match state.contains(&vector) {
                true if black == 0 || black > 2 => false,
                false if black == 2 => true,
                same => same,
            }
        })
        .collect()
}

pub fn part1(input: &[Vec<Vector>]) -> usize {
    initial_state(input).len()
}

pub fn part2(input: &[Vec<Vector>]) -> usize {
    let mut state = initial_state(input);
    for _ in 0..100 {
        state = next_state(state)
    }
    state.len()
}

#[test]
fn example() {
    let input = parse_input(include_str!("input/24_example.txt"));
    assert_eq!(part1(&input), 10);
    assert_eq!(part2(&input), 2208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 438);
    assert_eq!(part2(&input), 4038);
}
