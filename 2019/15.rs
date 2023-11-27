mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/15.txt"))
}

const NORTH: Vector2 = vector![0, 1];
const WEST: Vector2 = vector![-1, 0];
const SOUTH: Vector2 = vector![0, -1];
const EAST: Vector2 = vector![1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, SOUTH, WEST, EAST];

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
    fn next_status(&mut self, direction: Vector2) -> Option<Status> {
        self.input(match direction {
            NORTH => 1,
            SOUTH => 2,
            WEST => 3,
            EAST => 4,
            d => panic!("invalid direction `{d:?}`"),
        });
        self.next().map(|v| match v {
            0 => Status::Wall,
            1 => Status::Moved,
            2 => Status::Found,
            s => panic!("invalid status `{s}`"),
        })
    }
}

fn shortest(
    computer: &mut Computer,
    map: &mut HashMap<Vector2, Tile>,
    mut path: HashSet<Vector2>,
    pos: Vector2,
) -> usize {
    let mut min = usize::MAX;
    path.insert(pos);
    for d in CARDINALS {
        let next = pos + d;
        if map.contains_key(&next) || path.contains(&next) {
            continue;
        }
        match computer.next_status(d).unwrap() {
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

fn longest(map: &HashMap<Vector2, Tile>, mut path: HashSet<Vector2>, pos: Vector2) -> usize {
    let mut max = path.len();
    path.insert(pos);
    for d in CARDINALS {
        let next = pos + d;
        if path.contains(&next) || matches!(map.get(&next), Some(Tile::Wall)) {
            continue;
        }
        let dist = longest(map, path.clone(), next);
        max = cmp::max(max, dist);
    }
    max
}

fn solve(program: Vec<i64>) -> (HashMap<Vector2, Tile>, usize) {
    let mut computer = Computer::new(program);
    let mut map = HashMap::new();
    let min = shortest(&mut computer, &mut map, HashSet::new(), Vector2::zero());
    (map, min)
}

fn part1(program: Vec<i64>) -> usize {
    let (_, min) = solve(program);
    min
}

fn part2(program: Vec<i64>) -> usize {
    let (map, _) = solve(program);
    let pos = map
        .iter()
        .find_map(|(pos, tile)| matches!(tile, Tile::OxygenTank).then(|| *pos))
        .unwrap();
    longest(&map, HashSet::new(), pos)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 228);
    assert_eq!(part2(input), 348);
}
