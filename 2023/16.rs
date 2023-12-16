use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    parse_map(input, |c| match c {
        '.' => Tile::Empty,
        '|' => Tile::SplitVertical,
        '-' => Tile::SplitHorizontal,
        '/' => Tile::MirrorDiagonal,
        '\\' => Tile::MirrorAntidiagonal,
        c => panic!("unexpected character `{c}`"),
    })
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_input!(2023 / 16))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    SplitHorizontal,
    SplitVertical,
    MirrorAntidiagonal,
    MirrorDiagonal,
}

const UP: Vector2 = vector![0, -1];
const DOWN: Vector2 = vector![0, 1];
const LEFT: Vector2 = vector![-1, 0];
const RIGHT: Vector2 = vector![1, 0];

fn energized(layout: &HashMap<Vector2, Tile>, p: Vector2, d: Vector2) -> usize {
    let mut visited = HashSet::new();
    let mut q = VecDeque::from([(p, d)]);
    while let Some((p, d)) = q.pop_front() {
        if !layout.contains_key(&p) || !visited.insert((p, d)) {
            continue;
        }
        let next = match &layout[&p] {
            Tile::SplitVertical if d.y == 0 => Either::Left([UP, DOWN]),
            Tile::SplitHorizontal if d.x == 0 => Either::Left([LEFT, RIGHT]),
            Tile::MirrorDiagonal => Either::Right([vector![-d.y, -d.x]]),
            Tile::MirrorAntidiagonal => Either::Right([vector![d.y, d.x]]),
            _ => Either::Right([d]),
        }
        .into_iter();
        for d in next {
            q.push_back((p + d, d));
        }
    }
    let visited: HashSet<_> = visited.into_iter().map(|(p, _)| p).collect();
    visited.len()
}

fn part1(layout: HashMap<Vector2, Tile>) -> usize {
    energized(&layout, vector![0, 0], RIGHT)
}

fn part2(layout: HashMap<Vector2, Tile>) -> usize {
    let (min_x, max_x) = layout.keys().map(|p| p.x).min_max().unwrap();
    let (min_y, max_y) = layout.keys().map(|p| p.y).min_max().unwrap();
    let top = (min_x..=max_x).map(|x| (vector![x, 0], DOWN));
    let bottom = (min_x..=max_x).map(|x| (vector![x, max_y], UP));
    let left = (min_y..=max_y).map(|y| (vector![0, y], RIGHT));
    let right = (min_y..=max_y).map(|y| (vector![max_x, y], LEFT));
    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|(p, d)| energized(&layout, p, d))
        .max()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
    );
    assert_eq!(part1(input.clone()), 46);
    assert_eq!(part2(input), 51);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7067);
    assert_eq!(part2(input), 7324);
}
