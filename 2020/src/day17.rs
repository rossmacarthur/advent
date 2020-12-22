use std::collections::{HashMap, HashSet};
use std::iter;

use itertools::Itertools;
use vector::i64::Vector;

const INPUT: &str = include_str!("input/day17.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cube {
    Inactive,
    Active,
}

type Input = HashMap<(i64, i64), Cube>;
type Vector3 = Vector<3>;
type Vector4 = Vector<4>;

pub fn parse_input(s: &str) -> Input {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| match c {
                    '.' => Cube::Inactive,
                    '#' => Cube::Active,
                    c => panic!("unrecognized cube `{}`", c),
                })
                .enumerate()
                .map(move |(x, cube)| ((x as i64, y as i64), cube))
        })
        .collect()
}

pub fn default_input() -> Input {
    parse_input(INPUT)
}

fn neighbours<const N: usize>(center: Vector<N>) -> Vec<Vector<N>> {
    iter::repeat([-1, 0, 1].iter())
        .take(N)
        .multi_cartesian_product()
        .map(|v| v.into_iter().copied().collect())
        .filter(|&v| v != Vector::zero())
        .map(|dv: Vector<N>| center + dv)
        .collect()
}

fn neighbours_active<const N: usize>(state: &HashMap<Vector<N>, Cube>, center: Vector<N>) -> usize {
    neighbours(center)
        .into_iter()
        .filter_map(|vector| state.get(&vector))
        .filter(|cube| matches!(cube, Cube::Active))
        .count()
}

fn next_state<const N: usize>(state: HashMap<Vector<N>, Cube>) -> HashMap<Vector<N>, Cube> {
    let vectors: HashSet<_> = state.keys().copied().flat_map(neighbours).collect();
    let mut next = state.clone();
    for vector in vectors.into_iter() {
        let cube = state.get(&vector).unwrap_or(&Cube::Inactive);
        let active = neighbours_active(&state, vector);
        let next_cube = match cube {
            Cube::Active if matches!(active, 2 | 3) => Cube::Active,
            Cube::Active => Cube::Inactive,
            Cube::Inactive if active == 3 => Cube::Active,
            Cube::Inactive => Cube::Inactive,
        };
        next.insert(vector, next_cube);
    }
    next
}

fn active<const N: usize>(state: HashMap<Vector<N>, Cube>) -> usize {
    state
        .values()
        .filter(|cube| matches!(cube, Cube::Active))
        .count()
}

pub fn part1(input: &Input) -> usize {
    let mut state: HashMap<_, _> = input
        .iter()
        .map(|((x, y), cube)| (Vector3::from([*x, *y, 0]), *cube))
        .collect();
    for _ in 0..6 {
        state = next_state(state);
    }
    active(state)
}

pub fn part2(input: &HashMap<(i64, i64), Cube>) -> usize {
    let mut state: HashMap<_, _> = input
        .iter()
        .map(|((x, y), cube)| (Vector4::from([*x, *y, 0, 0]), *cube))
        .collect();
    for _ in 0..6 {
        state = next_state(state);
    }
    active(state)
}

#[test]
fn ext1() {
    let input = parse_input(".#.\n..#\n###");
    assert_eq!(part1(&input), 112);
    assert_eq!(part2(&input), 848);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 242);
    assert_eq!(part2(&input), 2292);
}
