use advent::prelude::*;

type Vector<const N: usize> = vectrix::Vector<i64, N>;
type State<const N: usize> = HashSet<Vector<N>>;

fn default_input() -> HashSet<Vector<2>> {
    parse_map_set(include_str!("input/17.txt"))
}

fn neighbours<const N: usize>(cube: Vector<N>) -> impl Iterator<Item = Vector<N>> {
    [[-1, 0, 1]; N]
        .into_iter()
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect::<Vector<N>>())
        .filter(|&v| v != Vector::zero())
        .map(move |dv| cube + dv)
}

fn neighbours_active<const N: usize>(state: &State<N>, cube: Vector<N>) -> usize {
    neighbours(cube).filter_map(|c| state.get(&c)).count()
}

fn next_state<const N: usize>(state: State<N>) -> State<N> {
    let neighbours: HashSet<_> = state.iter().copied().flat_map(neighbours).collect();
    neighbours
        .into_iter()
        .filter(|&cube| {
            let active = neighbours_active(&state, cube);
            matches!((state.contains(&cube), active), (true, 2 | 3) | (false, 3))
        })
        .collect()
}

fn solve<const N: usize>(input: HashSet<Vector<2>>) -> usize {
    let state = input
        .iter()
        .copied()
        .map(|cube| {
            let mut v = Vector::zero();
            v[0] = cube.x;
            v[1] = cube.y;
            v
        })
        .collect();
    (0..6).fold(state, |state, _| next_state::<N>(state)).len()
}

fn part1(input: HashSet<Vector<2>>) -> usize {
    solve::<3>(input)
}

fn part2(input: HashSet<Vector<2>>) -> usize {
    solve::<4>(input)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_map_set(".#.\n..#\n###");
    assert_eq!(part1(input.clone()), 112);
    assert_eq!(part2(input), 848);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 242);
    assert_eq!(part2(input), 2292);
}
