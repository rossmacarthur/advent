use std::collections::{hash_map::Entry, VecDeque};

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

fn part1(map: Map) -> usize {
    walk(&map).len()
}

fn part2(map: Map) -> usize {
    let Map { map: base_map, start: (start_p, start_d) } = map;
    let mut count = 0;
    let mut seen_obstacles = HashSet::new();
    let mut q = VecDeque::new();
    
    q.push_back((start_p, start_d));

    while let Some((p, d)) = q.pop_front() {
        let next = p + d;

        match base_map.get(&next) {
            None => continue,
            Some(Tile::Empty | Tile::Start) => {
                // If this is an empty space we could use as an obstacle
                if next != start_p && !seen_obstacles.contains(&next) {
                    seen_obstacles.insert(next);
                    
                    // Start a fresh traversal with just this obstacle
                    let mut visited = HashSet::new();
                    let mut test_q = VecDeque::new();
                    test_q.push_back((p, vector![-d.y, d.x]));  // Turn at the obstacle
                    
                    while let Some((test_p, test_d)) = test_q.pop_front() {
                        if !visited.insert((test_p, test_d)) {
                            count += 1;
                            break;
                        }
                        let test_next = test_p + test_d;
                        if test_next == next {
                            // Hit our obstacle, turn
                            test_q.push_back((test_p, vector![-test_d.y, test_d.x]));
                        } else {
                            match base_map.get(&test_next) {
                                None => break,
                                Some(Tile::Empty | Tile::Start) => {
                                    test_q.push_back((test_next, test_d));
                                }
                                Some(Tile::Obstacle) => {
                                    test_q.push_back((test_p, vector![-test_d.y, test_d.x]));
                                }
                            }
                        }
                    }
                }
                // Continue main traversal
                q.push_back((next, d));
            }
            Some(Tile::Obstacle) => {
                q.push_back((p, vector![-d.y, d.x]));
            }
        }
    }
    count
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
