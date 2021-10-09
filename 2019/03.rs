use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex_macro::regex;
use vectrix::{vector, Vector2};

type Vector = Vector2<i64>;
type Path = Vec<(Vector, i64)>;

fn parse_input(input: &str) -> (Path, Path) {
    input
        .lines()
        .map(|line| {
            regex!(r"(L|R|D|U)(\d+)")
                .captures_iter(line)
                .map(|caps| {
                    let length: i64 = caps[2].parse().unwrap();
                    let direction = match &caps[1] {
                        "L" => vector![-1, 0],
                        "R" => vector![1, 0],
                        "D" => vector![0, -1],
                        "U" => vector![0, 1],
                        _ => unreachable!(),
                    };
                    (direction, length)
                })
                .collect()
        })
        .next_tuple()
        .unwrap()
}

fn default_input() -> (Path, Path) {
    parse_input(include_str!("input/03.txt"))
}

fn distances(path: &[(Vector, i64)]) -> HashMap<Vector, i64> {
    let mut distances = HashMap::new();
    let mut position = Vector::zero();
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

fn part1((path1, path2): &(Path, Path)) -> i64 {
    keys(&distances(path1))
        .intersection(&keys(&distances(path2)))
        .map(Vector::l1_norm)
        .min()
        .unwrap()
}

fn part2((p1, p2): &(Path, Path)) -> i64 {
    let distances1 = distances(p1);
    let distances2 = distances(p2);
    keys(&distances1)
        .intersection(&keys(&distances2))
        .into_iter()
        .map(|position| distances1[position] + distances2[position])
        .min()
        .unwrap()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input("R8,U5,L5,D3\nU7,R6,D4,L4");
    assert_eq!(part1(&input), 6);
    assert_eq!(part2(&input), 30);
}

#[test]
fn example2() {
    let input = parse_input(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
         U62,R66,U55,R34,D71,R55,D58,R83",
    );
    assert_eq!(part1(&input), 159);
    assert_eq!(part2(&input), 610);
}

#[test]
fn example3() {
    let input = parse_input(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );
    assert_eq!(part1(&input), 135);
    assert_eq!(part2(&input), 410);
}
