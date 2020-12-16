use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use vector::Vector;

const INPUT: &str = include_str!("input/day10.txt");

pub fn default_input() -> HashSet<Vector> {
    INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Vector::two(x as i64, y as i64))
        })
        .collect()
}

fn visible(asteroids: &HashSet<Vector>, center: Vector) -> HashMap<Vector, Vec<(i64, Vector)>> {
    let mut visible = HashMap::new();
    for asteroid in asteroids.iter().copied() {
        if asteroid == center {
            continue;
        }
        let dv = asteroid - center;
        let element = (dv.manhattan_distance(), asteroid);
        let vec = visible.entry(dv.reduced()).or_insert_with(Vec::new);
        let pos = vec.binary_search(&element).unwrap_err();
        vec.insert(pos, element);
    }
    visible
}

pub fn part1(asteroids: &HashSet<Vector>) -> usize {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| visible(asteroids, asteroid).len())
        .max()
        .unwrap()
}

pub fn part2(asteroids: &HashSet<Vector>) -> i64 {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| (visible(asteroids, asteroid), asteroid))
        .max_by_key(|(visible, _)| visible.len())
        .unwrap()
        .0
        .into_iter()
        .map(|(vector, mut asteroids)| (vector.rotated(90).angle(), asteroids.remove(0).1))
        .sorted_by(|(angle1, _), (angle2, _)| angle1.partial_cmp(angle2).unwrap())
        .nth(199)
        .map(|(_, asteroid)| asteroid.x * 100 + asteroid.y)
        .unwrap()
}
