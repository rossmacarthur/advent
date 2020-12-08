use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex_macro::regex;

const INPUT: &str = include_str!("input/day03.txt");

type Path = Vec<(Direction, isize)>;

type Point = (isize, isize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

pub fn default_input() -> (Path, Path) {
    INPUT
        .lines()
        .map(|line| {
            regex!(r"(L|R|D|U)(\d+)")
                .captures_iter(line)
                .map(|caps| {
                    let length: isize = caps[2].parse().unwrap();
                    let direction = match &caps[1] {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "U" => Direction::Up,
                        _ => unreachable!(),
                    };
                    (direction, length)
                })
                .collect()
        })
        .next_tuple()
        .unwrap()
}

fn distances(path: &Path) -> HashMap<Point, isize> {
    let mut points = HashMap::new();
    let mut position = (0, 0);
    let mut distance = 0;
    for &(direction, length) in path {
        let vector = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Up => (0, 1),
        };
        for _ in 0..length {
            position.0 += vector.0;
            position.1 += vector.1;
            distance += 1;
            points.insert(position, distance);
        }
    }
    points
}

fn keys(points: &HashMap<Point, isize>) -> HashSet<Point> {
    points.iter().map(|(k, _)| *k).collect()
}

pub fn part1((path1, path2): &(Path, Path)) -> isize {
    keys(&distances(&path1))
        .intersection(&keys(&distances(&path2)))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

pub fn part2((p1, p2): &(Path, Path)) -> isize {
    let distances1 = distances(p1);
    let distances2 = distances(p2);
    keys(&distances1)
        .intersection(&keys(&distances2))
        .into_iter()
        .map(|point| distances1[point] + distances2[point])
        .min()
        .unwrap()
}
