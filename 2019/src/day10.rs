use std::collections::{HashMap, HashSet};
use std::f64::consts::{FRAC_PI_2, TAU};
use std::ops::Add;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day10.txt");

type Point = (i32, i32);

pub fn default_input() -> HashSet<Point> {
    INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

fn angle(vector: Point) -> f64 {
    let dx = vector.0 as f64;
    let dy = vector.1 as f64;
    dy.atan2(dx).add(FRAC_PI_2).rem_euclid(TAU)
}

fn vector(a: Point, b: Point) -> Point {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let div = gcd(dx, dy);
    (
        dx.checked_div(div).unwrap_or(0),
        dy.checked_div(div).unwrap_or(0),
    )
}

fn manhatten_distance(a: Point, b: Point) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn visible(asteroids: &HashSet<Point>, center: Point) -> HashMap<Point, Vec<(i32, Point)>> {
    let mut visible = HashMap::new();
    for asteroid in asteroids.iter().copied() {
        if asteroid == center {
            continue;
        }
        let vec = visible
            .entry(vector(center, asteroid))
            .or_insert_with(Vec::new);
        let element = (manhatten_distance(center, asteroid), asteroid);
        let pos = vec.binary_search(&element).unwrap_err();
        vec.insert(pos, element);
    }
    visible
}

pub fn part1(asteroids: &HashSet<Point>) -> usize {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| visible(asteroids, asteroid).len())
        .max()
        .unwrap()
}

pub fn part2(asteroids: &HashSet<Point>) -> i32 {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| (visible(asteroids, asteroid), asteroid))
        .max_by_key(|(visible, _)| visible.len())
        .unwrap()
        .0
        .into_iter()
        .map(|(vector, mut asteroids)| (angle(vector), asteroids.remove(0).1))
        .sorted_by(|(a1, _), (a2, _)| a1.partial_cmp(a2).unwrap())
        .nth(199)
        .map(|(_, asteroid)| asteroid.0 * 100 + asteroid.1)
        .unwrap()
}
