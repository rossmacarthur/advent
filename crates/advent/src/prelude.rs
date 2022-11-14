pub use std::cmp;
pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
pub use std::iter;
pub use std::mem;

pub use either::Either;
pub use itermore::Itermore;
pub use itertools::{iproduct, Itertools};
pub use regex_macro::regex;
pub use some::Some;
pub use vectrix::{vector, Matrix, Vector};

pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;
pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

pub type Vector2 = vectrix::Vector<i64, 2>;
pub type Vector3 = vectrix::Vector<i64, 3>;

pub trait CollectionExt {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V> CollectionExt for HashMap<K, V> {
    #[inline]
    fn new() -> Self {
        HashMap::with_hasher(ahash::RandomState::new())
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, ahash::RandomState::new())
    }
}

impl<T> CollectionExt for HashSet<T> {
    #[inline]
    fn new() -> Self {
        HashSet::with_hasher(ahash::RandomState::new())
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity_and_hasher(capacity, ahash::RandomState::new())
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

/// Parses a 2D map into a data structure of type `M`.
pub fn parse_map<M, F, V>(input: &str, parse: F) -> M
where
    M: FromIterator<(Vector2, V)>,
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

/// Parses a 2D map that uses '.' for spaces and '#' for walls as a set of the
/// the '#' points.
pub fn parse_map_set(input: &str) -> HashSet<Vector2> {
    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '#' => Some(()),
        '.' => None,
        c => panic!("unrecognized character `{}`", c),
    });
    map.into_iter().filter_map(|(k, v)| v.map(|_| k)).collect()
}
