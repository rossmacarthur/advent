use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex_macro::regex;
use vector::Vector;

const INPUT: &str = include_str!("input/day03.txt");

pub fn default_input() -> (Path, Path) {
    INPUT
        .lines()
        .map(|line| {
            regex!(r"(L|R|D|U)(\d+)")
                .captures_iter(line)
                .map(|caps| {
                    let length: i64 = caps[2].parse().unwrap();
                    let direction = match &caps[1] {
                        "L" => Vector::new(-1, 0),
                        "R" => Vector::new(1, 0),
                        "D" => Vector::new(0, -1),
                        "U" => Vector::new(0, 1),
                        _ => unreachable!(),
                    };
                    (direction, length)
                })
                .collect()
        })
        .next_tuple()
        .unwrap()
}

type Path = Vec<(Vector, i64)>;

fn distances(path: &[(Vector, i64)]) -> HashMap<Vector, i64> {
    let mut distances = HashMap::new();
    let mut position = Vector::new(0, 0);
    let mut distance = 0;
    for &(direction, length) in path {
        for _ in 0..length {
            position += direction;
            distance += 1;
            distances.insert(position, distance);
        }
    }
    distances
}

fn keys(distances: &HashMap<Vector, i64>) -> HashSet<Vector> {
    distances.iter().map(|(k, _)| *k).collect()
}

pub fn part1((path1, path2): &(Path, Path)) -> i64 {
    keys(&distances(&path1))
        .intersection(&keys(&distances(&path2)))
        .map(Vector::manhattan_distance)
        .min()
        .unwrap()
}

pub fn part2((p1, p2): &(Path, Path)) -> i64 {
    let distances1 = distances(p1);
    let distances2 = distances(p2);
    keys(&distances1)
        .intersection(&keys(&distances2))
        .into_iter()
        .map(|position| distances1[position] + distances2[position])
        .min()
        .unwrap()
}
