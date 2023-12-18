use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    parse_map(input, |c| match c {
        '.' => Tile::Ground,
        'S' => Tile::Start,
        '|' => Tile::Pipe([NORTH, SOUTH]),
        '-' => Tile::Pipe([EAST, WEST]),
        'L' => Tile::Pipe([NORTH, EAST]),
        'J' => Tile::Pipe([NORTH, WEST]),
        '7' => Tile::Pipe([SOUTH, WEST]),
        'F' => Tile::Pipe([SOUTH, EAST]),
        c => panic!("unexpected character `{c}`"),
    })
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_input!(2023 / 10))
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Ground,
    Start,
    Pipe([Vector2; 2]),
}

const NORTH: Vector2 = vector![0, -1];
const WEST: Vector2 = vector![-1, 0];
const SOUTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];

/// Finds the first point after the starting point on the loop and returns the
/// location and the direction from start to that point.
fn find_start(field: &HashMap<Vector2, Tile>) -> (Vector2, Vector2) {
    let start = field
        .iter()
        .find_map(|(&p, &tile)| (tile == Tile::Start).some(p))
        .unwrap();
    [NORTH, SOUTH, WEST, EAST]
        .into_iter()
        .find_map(|d| {
            let p = start + d;
            match field.get(&p).copied() {
                Some(Tile::Pipe([d1, d2])) if d == -d1 || d == -d2 => Some((p, d)),
                _ => None,
            }
        })
        .unwrap()
}

/// Visits the entire loop from the starting point and returns the path.
fn visit_loop(field: HashMap<Vector2, Tile>) -> Vec<Vector2> {
    let mut path = vec![find_start(&field)];
    while let Some((p, d)) = path.last() {
        let next_d = match field[p] {
            Tile::Start => break,
            Tile::Pipe([d1, d2]) if d1 == -d => d2,
            Tile::Pipe([d2, d1]) if d1 == -d => d2,
            _ => panic!("broken path"),
        };
        path.push((p + next_d, next_d));
    }
    path.into_iter().map(|(p, _)| p).collect()
}

fn part1(field: HashMap<Vector2, Tile>) -> usize {
    visit_loop(field).len() / 2
}

fn part2(field: HashMap<Vector2, Tile>) -> i64 {
    let path = visit_loop(field);
    let mut s = 0;
    let b = path.len() as i64;
    for [p, q] in path.into_iter().circular_array_windows() {
        s += p.x * q.y - q.x * p.y;
    }
    (s.abs() - b) / 2 + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
    );
    assert_eq!(part1(input), 4);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
    );
    assert_eq!(part1(input), 8);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
    );
    assert_eq!(part2(input), 4);
}

#[test]
fn example4() {
    let input = parse_input(
        "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
    );

    assert_eq!(part2(input), 8);
}

#[test]
fn example5() {
    let input = parse_input(
        "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    );
    assert_eq!(part2(input), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6947);
    assert_eq!(part2(input), 273);
}
