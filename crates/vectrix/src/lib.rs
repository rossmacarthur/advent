use std::collections::{HashMap, HashSet};

pub use vectrix::*;

pub type Vector2<T> = vectrix::Vector<T, 2>;
pub type Vector3<T> = vectrix::Vector<T, 3>;

pub const NORTH: Vector2<i64> = vector![0, 1];
pub const SOUTH: Vector2<i64> = vector![0, -1];
pub const WEST: Vector2<i64> = vector![-1, 0];
pub const EAST: Vector2<i64> = vector![1, 0];
pub const CARDINALS: &[Vector2<i64>] = &[NORTH, SOUTH, WEST, EAST];

pub trait VectorExt {
    fn reduced(self) -> Self;
    fn rotated(self, angle: i64) -> Self;
    fn angle(&self) -> f64;
}

impl VectorExt for Vector2<i64> {
    /// Returns the reduced row echelon form of the vector.
    ///
    /// This is the same as dividing each element by the greatest common
    /// divisor of all the elements.
    fn reduced(self) -> Self {
        let x = self.x;
        let y = self.y;
        let div = gcd(x, y);
        vector![x / div, y / div]
    }

    /// Returns the vector rotated around the origin.
    ///
    /// # Panics
    ///
    /// If the angle is not a multiple of 90 degrees.
    fn rotated(self, angle: i64) -> Self {
        let x = self.x;
        let y = self.y;
        match angle.rem_euclid(360) {
            0 => vector![x, y],
            90 => vector![-y, x],
            180 => vector![-x, -y],
            270 => vector![y, -x],
            angle => panic!("called `Vector::rotate()` with oblique angle `{}`", angle),
        }
    }

    /// Returns the angle in radians between the vector and the origin.
    fn angle(&self) -> f64 {
        (self.y as f64)
            .atan2(self.x as f64)
            .rem_euclid(std::f64::consts::TAU)
    }
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

pub fn parse_map<F, V>(input: &str, parse: F) -> HashMap<Vector2<i64>, V>
where
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

pub fn parse_map_set(input: &str) -> HashSet<Vector2<i64>> {
    parse_map(input, |c| match c {
        '#' => Some(()),
        '.' => None,
        _ => panic!("unrecognized character"),
    })
    .into_iter()
    .filter_map(|(k, v)| v.map(|_| k))
    .collect()
}
