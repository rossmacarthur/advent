mod intcode;

use std::cmp;
use std::collections::{HashMap, HashSet};

use vectrix::{vector, Vector2};

use intcode::{parse_program, Computer};

type Vector = Vector2<i64>;

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/15.txt"))
}

const NORTH: Vector = vector![0, 1];
const SOUTH: Vector = vector![0, -1];
const WEST: Vector = vector![-1, 0];
const EAST: Vector = vector![1, 0];
const DIRECTIONS: &[Vector] = &[NORTH, SOUTH, WEST, EAST];

enum Status {
    Wall,
    Moved,
    Found,
}

enum Tile {
    Floor,
    Wall,
    OxygenTank,
}

impl Computer {
    fn next_status(&mut self, direction: Vector) -> Option<Status> {
        self.input(match direction {
            NORTH => 1,
            SOUTH => 2,
            WEST => 3,
            EAST => 4,
            d => panic!("invalid direction `{:?}`", d),
        });
        self.next_value().map(|v| match v {
            0 => Status::Wall,
            1 => Status::Moved,
            2 => Status::Found,
            s => panic!("invalid status `{}`", s),
        })
    }
}

fn shortest(
    computer: &mut Computer,
    map: &mut HashMap<Vector, Tile>,
    mut path: HashSet<Vector>,
    pos: Vector,
) -> usize {
    let mut min = usize::MAX;
    path.insert(pos);
    for d in DIRECTIONS {
        let next = pos + d;
        if map.contains_key(&next) || path.contains(&next) {
            continue;
        }
        match computer.next_status(*d).unwrap() {
            Status::Wall => {
                map.insert(next, Tile::Wall);
                continue;
            }
            Status::Found => {
                map.insert(next, Tile::OxygenTank);
                min = cmp::min(min, path.len());
            }
            Status::Moved => {
                map.insert(next, Tile::Floor);
                let dist = shortest(computer, map, path.clone(), next);
                min = cmp::min(min, dist);
            }
        }
        computer.next_status(-d).unwrap();
    }
    min
}

fn longest(map: &HashMap<Vector, Tile>, mut path: HashSet<Vector>, pos: Vector) -> usize {
    let mut max = path.len();
    path.insert(pos);
    for d in DIRECTIONS {
        let next = pos + d;
        if path.contains(&next) || matches!(map.get(&next), Some(Tile::Wall)) {
            continue;
        }
        let dist = longest(map, path.clone(), next);
        max = cmp::max(max, dist);
    }
    max
}

fn solve(program: Vec<i64>) -> (HashMap<Vector, Tile>, usize) {
    let mut computer = Computer::new(program);
    let mut map = HashMap::new();
    let min = shortest(&mut computer, &mut map, HashSet::new(), Vector::zero());
    (map, min)
}

fn part1(program: Vec<i64>) -> usize {
    solve(program).1
}

fn part2(program: Vec<i64>) -> usize {
    let (map, _) = solve(program);
    let pos = map
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::OxygenTank))
        .map(|(pos, _)| *pos)
        .unwrap();
    longest(&map, HashSet::new(), pos)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(input.clone()));
    run.part(|| part2(input.clone()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 228);
    assert_eq!(part2(input), 348);
}
