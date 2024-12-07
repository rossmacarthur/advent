use std::collections::hash_map::Entry;

use advent::prelude::*;

fn parse_input(input: &str) -> Map {
    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Obstacle,
        '^' => Tile::Start,
        c => panic!("unexpected character `{c}`"),
    });
    let p = map
        .iter()
        .find_map(|(&p, &tile)| (tile == Tile::Start).some(p))
        .unwrap();
    let d = vector![0, -1];

    Map { map, start: (p, d) }
}

fn default_input() -> Map {
    parse_input(include_input!(2024 / 06))
}

#[derive(Debug, Clone)]
struct Map {
    map: HashMap<Vector2, Tile>,
    start: (Vector2, Vector2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Obstacle,
}

fn walk(map: &Map) -> HashSet<Vector2> {
    let Map { map, start } = map;
    let (mut p, mut d) = *start;

    let mut visited = HashSet::new();
    loop {
        visited.insert(p);
        match map.get(&(p + d)) {
            None => break,
            Some(Tile::Empty | Tile::Start) => p += d,
            Some(Tile::Obstacle) => d = vector![-d.y, d.x],
        }
    }
    visited
}

fn loops(map: &Map, obstacle: Vector2) -> bool {
    let Map { map, start } = map;
    let (mut p, mut d) = *start;

    let mut visited = HashSet::new();
    loop {
        if !visited.insert((p, d)) {
            return true;
        }
        let next = p + d;
        if next == obstacle {
            d = vector![-d.y, d.x];
            continue;
        }
        match map.get(&next) {
            None => break,
            Some(Tile::Empty | Tile::Start) => p += d,
            Some(Tile::Obstacle) => d = vector![-d.y, d.x],
        }
    }
    false
}

fn part1(map: Map) -> usize {
    walk(&map).len()
}

fn part2(map: Map) -> usize {
    walk(&map)
        .into_iter()
        .filter(|&obstacle| obstacle != map.start.0 && loops(&map, obstacle))
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    );
    assert_eq!(part1(input.clone()), 41);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4602);
    assert_eq!(part2(input), 1703);
}
