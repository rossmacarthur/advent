mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/17.txt"))
}

impl Computer {
    fn read_image(&mut self) -> Option<HashMap<Vector2, char>> {
        let mut map = HashMap::new();
        let mut y = 0;
        loop {
            match self.read_line().as_deref()? {
                "" => break Some(map),
                line => {
                    map.extend(line.chars().enumerate().filter_map(|(x, c)| match c {
                        '.' => None,
                        c => Some((vector![x as i64, y], c)),
                    }));
                    y += 1;
                }
            }
        }
    }
}

const NORTH: Vector2 = vector![0, -1];
const SOUTH: Vector2 = vector![0, 1];
const WEST: Vector2 = vector![-1, 0];
const EAST: Vector2 = vector![1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, SOUTH, WEST, EAST];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Left(i64),
    Right(i64),
}

impl Move {
    fn to_ascii(self) -> String {
        match self {
            Self::Left(d) => format!("L,{d}"),
            Self::Right(d) => format!("R,{d}"),
        }
    }
}

/// Returns the set of scaffold locations.
fn scaffolds(image: &HashMap<Vector2, char>) -> HashSet<Vector2> {
    image
        .iter()
        .filter_map(|(v, p)| (*p == '#').some(*v))
        .collect()
}

fn navigate(image: &HashMap<Vector2, char>) -> Vec<Move> {
    // Find the droid and the direction it is facing
    let (mut droid, mut d) = image
        .iter()
        .find_map(|(v, c)| match c {
            '<' => Some((*v, WEST)),
            '>' => Some((*v, EAST)),
            '^' => Some((*v, NORTH)),
            'v' => Some((*v, SOUTH)),
            _ => None,
        })
        .unwrap();

    let mut moves = Vec::new();

    // Given the droid position and a direction, navigate to the next scaffold.
    let scaffolds = scaffolds(image);
    let mut mv = |droid: Vector2, d: Vector2| {
        // Try continue in the same direction.
        let next = droid + d;
        if scaffolds.contains(&next) {
            match moves.last_mut().unwrap() {
                Move::Left(n) | Move::Right(n) => *n += 1,
            }
            Some((next, d))
        // Otherwise we need to find the best way to go, which must be a
        // scaffold and not our previous position.
        } else {
            let prev = droid - d;
            for to in CARDINALS {
                let next = droid + to;
                if scaffolds.contains(&next) && next != prev {
                    moves.push(match d.x * to.y + to.x * -d.y {
                        -1 => Move::Left(1),
                        1 => Move::Right(1),
                        _ => unreachable!(),
                    });
                    return Some((next, to));
                }
            }
            None
        }
    };

    while let Some((next, to)) = mv(droid, d) {
        droid = next;
        d = to;
    }
    moves
}

/// Returns whether the function is valid and is usable more than once.
fn is_valid_function(f: &[Move], positions: &[usize]) -> bool {
    positions.len() > 1 && (f.iter().map(|m| m.to_ascii()).join(",").len() <= 20)
}

/// Returns whether the set of functions is a valid routine.
fn is_valid_routine(moves: &[Move], routine: &[(&[Move], Vec<usize>)]) -> bool {
    let all: HashSet<_> = (0..moves.len()).collect();
    let indexes: Vec<_> = routine
        .iter()
        .map(|(f, positions)| {
            let indexes: HashSet<_> = positions.iter().flat_map(|&p| p..(p + f.len())).collect();
            indexes
        })
        .collect();
    indexes
        .iter()
        .array_windows()
        .all(|[a, b]| a.is_disjoint(b))
        && indexes
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .is_superset(&all)
}

/// Returns a function routine from a list of moves.
fn routine(moves: &[Move]) -> (String, Vec<String>) {
    // First we find all possible functions.
    //
    // This is done by taking windows of function sizes one to eight. We filter
    // for valid length functions and functions that can be used more than once.
    // We only go up to eight as this is the maximum number of moves a function
    // can have.
    let functions: HashMap<_, _> = (1..8)
        .flat_map(|w| {
            moves.windows(w).map(move |f| {
                let positions: Vec<_> = moves.windows(w).positions(|w| w == f).collect();
                (f, positions)
            })
        })
        .filter(|(f, positions)| is_valid_function(f, positions))
        .collect();
    // Then we sort by function length, then number of times it can be used.
    // This will help us find a valid combination faster.
    let functions = functions
        .into_iter()
        .sorted_by_key(|(f, positions)| Reverse((f.len(), positions.len())));
    // Now we find a valid routine.
    //
    // A routine is valid if all moves can be made using it and there are no
    // overlapping functions.
    let routine: [(&[Move], Vec<usize>); 3] = functions
        .array_combinations::<3>()
        .find(|routine| is_valid_routine(moves, routine))
        .unwrap();
    // Finally, we have the three functions and the non-overlapping ranges of
    // moves that they occur at. So we just assign "A", "B", and "C" to each
    // function and sort the positions to figure out the order to call the
    // functions.
    let mut functions = Vec::new();
    let routine = "ABC"
        .chars()
        .zip(routine)
        .flat_map(|(name, (f, positions))| {
            functions.push(f.iter().map(|m| m.to_ascii()).join(","));
            positions.into_iter().map(move |p| (name, p))
        })
        .sorted_by_key(|(_, p)| *p)
        .map(|(name, _)| name)
        .join(",");

    (routine, functions)
}

fn part1(program: Vec<i64>) -> i64 {
    let mut c = Computer::new(program);
    let image = c.read_image().unwrap();
    let scaffolds = scaffolds(&image);
    scaffolds
        .iter()
        .filter(|&s| CARDINALS.into_iter().all(|d| scaffolds.contains(&(s + d))))
        .map(|s| s.x * s.y)
        .sum()
}

fn part2(mut program: Vec<i64>) -> i64 {
    program[0] = 2;
    let mut c = Computer::new(program);
    let image = c.read_image().unwrap();
    let moves = navigate(&image);
    let (routine, functions) = routine(&moves);
    c.read_line().unwrap();
    c.write_line(&routine);
    c.read_line().unwrap();
    for f in functions {
        c.write_line(&f);
        c.read_line().unwrap();
    }
    c.write_line("n");
    c.read_line().unwrap();
    c.read_image().unwrap();
    c.next().unwrap()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2508);
    assert_eq!(part2(input), 799463);
}
