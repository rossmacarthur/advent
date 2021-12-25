use hashbrown::HashMap;
use vectrix::{parse_map, vector, Vector2};

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> HashMap<Vector, Tile> {
    parse_map(input, |c| match c {
        '>' => Tile::Occupied(Cucumber::East),
        'v' => Tile::Occupied(Cucumber::South),
        '.' => Tile::Empty,
        c => panic!("unexpected character `{}`", c),
    })
}

fn default_input() -> HashMap<Vector, Tile> {
    parse_input(include_str!("input/25.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Occupied(Cucumber),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cucumber {
    East,
    South,
}

impl Cucumber {
    const fn direction(&self) -> Vector {
        match *self {
            Self::East => vector![1, 0],
            Self::South => vector![0, 1],
        }
    }

    fn wrap(&self, p: Vector) -> Vector {
        match *self {
            Self::East => vector![0, p.y],
            Self::South => vector![p.x, 0],
        }
    }
}

fn move_herd(map: &mut HashMap<Vector, Tile>, cucumber: Cucumber) -> bool {
    let moves: Vec<_> = map
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Occupied(c) if *c == cucumber))
        .filter_map(|(&p, _)| {
            let n = p + cucumber.direction();
            match map.get(&n) {
                Some(Tile::Empty) => Some((p, n)),
                Some(_) => None,
                None => {
                    let n = cucumber.wrap(p);
                    match map.get(&n) {
                        Some(Tile::Empty) => Some((p, n)),
                        _ => None,
                    }
                }
            }
        })
        .collect();
    for &(p, n) in &moves {
        map.insert(p, Tile::Empty);
        map.insert(n, Tile::Occupied(cucumber));
    }
    !moves.is_empty()
}

fn part1(mut map: HashMap<Vector, Tile>) -> i64 {
    let mut step = 1;
    loop {
        let moved_east = move_herd(&mut map, Cucumber::East);
        let moved_south = move_herd(&mut map, Cucumber::South);
        if !moved_east && !moved_south {
            break step;
        }
        step += 1;
    }
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
    );
    assert_eq!(part1(input), 58);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 426);
}
