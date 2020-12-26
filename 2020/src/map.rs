use std::collections::{HashMap, HashSet};

use vector::i64::xy::Vector;

pub fn parse_map<F, V>(input: &str, parse: F) -> HashMap<Vector, V>
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
                .map(move |(x, v)| (Vector::new([x as i64, y as i64]), v))
        })
        .collect()
}

pub fn parse_map_set(input: &str) -> HashSet<Vector> {
    parse_map(input, |c| match c {
        '#' => Some(()),
        '.' => None,
        _ => panic!("unrecognized character"),
    })
    .into_iter()
    .filter_map(|(k, v)| v.map(|_| k))
    .collect()
}
