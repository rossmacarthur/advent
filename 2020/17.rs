use std::collections::HashSet;
use std::iter;

use itertools::Itertools;
use vectrix::parse_map_set;

type Vector<const N: usize> = vectrix::Vector<i64, N>;
type State<const N: usize> = HashSet<Vector<N>>;

fn default_input() -> HashSet<Vector<2>> {
    parse_map_set(include_str!("input/17.txt"))
}

fn neighbours<const N: usize>(center: Vector<N>) -> Vec<Vector<N>> {
    iter::repeat([-1, 0, 1].iter().copied())
        .take(N)
        .multi_cartesian_product()
        .map(|v| {
            assert_eq!(v.len(), N);
            let mut vector = Vector::default();
            for i in 0..N {
                vector[i] = v[i]
            }
            vector
        })
        .filter(|&v| v != Vector::zero())
        .map(|dv: Vector<N>| center + dv)
        .collect()
}

fn neighbours_active<const N: usize>(state: &State<N>, center: Vector<N>) -> usize {
    neighbours(center)
        .into_iter()
        .filter_map(|vector| state.get(&vector))
        .count()
}

fn next_state<const N: usize>(state: State<N>) -> State<N> {
    state
        .iter()
        .copied()
        .flat_map(neighbours)
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&vector| {
            let active = neighbours_active(&state, vector);
            match state.contains(&vector) {
                true if (2..4).contains(&active) => true,
                false if (3..4).contains(&active) => true,
                _ => false,
            }
        })
        .collect()
}

fn solve<const N: usize>(input: &HashSet<Vector<2>>) -> usize {
    let state = input
        .iter()
        .copied()
        .map(|v| {
            let mut vector = Vector::default();
            vector[0] = v.x;
            vector[1] = v.y;
            vector
        })
        .collect();
    (0..6).fold(state, |state, _| next_state::<N>(state)).len()
}

fn part1(input: &HashSet<Vector<2>>) -> usize {
    solve::<3>(input)
}

fn part2(input: &HashSet<Vector<2>>) -> usize {
    solve::<4>(input)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_map_set(".#.\n..#\n###");
    assert_eq!(part1(&input), 112);
    assert_eq!(part2(&input), 848);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 242);
    assert_eq!(part2(&input), 2292);
}
