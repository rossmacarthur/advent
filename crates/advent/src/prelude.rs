pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
pub use std::iter;
pub use std::mem;

pub use either::Either;
pub use hashbrown::{HashMap, HashSet};
pub use itermore::Itermore;
pub use itertools::Itertools;
pub use regex_macro::regex;
pub use vectrix::{vector, Vector};

pub type Vector2 = vectrix::Vector<i64, 2>;
pub type Vector3 = vectrix::Vector<i64, 3>;

/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}
