use std::collections::HashMap;

pub use vectrix::*;

pub type Vector2<T> = vectrix::Vector<T, 2>;
pub type Vector3<T> = vectrix::Vector<T, 3>;

pub const NORTH: Vector2<i64> = vector![0, 1];
pub const NORTH_WEST: Vector2<i64> = vector![-1, 1];
pub const WEST: Vector2<i64> = vector![-1, 0];
pub const SOUTH_WEST: Vector2<i64> = vector![-1, -1];
pub const SOUTH: Vector2<i64> = vector![0, -1];
pub const SOUTH_EAST: Vector2<i64> = vector![1, -1];
pub const EAST: Vector2<i64> = vector![1, 0];
pub const NORTH_EAST: Vector2<i64> = vector![1, 1];

pub const CARDINALS: &[Vector2<i64>] = &[NORTH, EAST, SOUTH, WEST];
pub const CARDINALS_8: &[Vector2<i64>] = &[
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

pub fn parse_map<M, F, V>(input: &str, parse: F) -> M
where
    M: FromIterator<(Vector2<i64>, V)>,
    F: Fn(char) -> V,
{
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (x, parse(c)))
                .map(move |(x, v)| (vector![x as i64, y as i64], v))
        })
        .collect()
}

pub fn parse_map_set<S>(input: &str) -> S
where
    S: FromIterator<Vector2<i64>>,
{
    let m: HashMap<_, _> = parse_map(input, |c| match c {
        '#' => Some(()),
        '.' => None,
        _ => panic!("unrecognized character"),
    });
    m.into_iter().filter_map(|(k, v)| v.map(|_| k)).collect()
}
