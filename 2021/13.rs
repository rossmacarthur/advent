use std::collections::HashSet;

use vectrix::{vector, Vector2};

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> (HashSet<Vector>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            vector![x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();
    let folds = folds
        .lines()
        .map(|line| {
            let (axis, value) = line.split_once('=').unwrap();
            let axis = match axis.chars().last().unwrap() {
                'x' => 0,
                'y' => 1,
                axis => panic!("unknown axis `{}`", axis),
            };
            let value = value.parse().unwrap();
            Fold { axis, value }
        })
        .collect();
    (dots, folds)
}

fn default_input() -> (HashSet<Vector>, Vec<Fold>) {
    parse_input(include_str!("input/13.txt"))
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    axis: usize,
    value: i64,
}

fn fold(dots: HashSet<Vector>, Fold { axis, value }: Fold) -> HashSet<Vector> {
    dots.into_iter()
        .map(|mut v| {
            v[axis] = value - (v[axis] - value).abs();
            v
        })
        .collect()
}

fn part1((dots, folds): (HashSet<Vector>, Vec<Fold>)) -> usize {
    fold(dots, folds[0]).len()
}

fn part2((mut dots, folds): (HashSet<Vector>, Vec<Fold>)) -> String {
    for f in folds {
        dots = fold(dots, f);
    }

    let mut code = String::new();
    let max_x = dots.iter().map(|v| v.x).max().unwrap();
    let max_y = dots.iter().map(|v| v.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            code += match dots.contains(&vector![x, y]) {
                false => "░░",
                true => "██",
            };
        }
        code += "\n";
    }
    code
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
    );
    assert_eq!(part1(input.clone()), 17);
    assert_eq!(
        part2(input),
        "\
██████████
██░░░░░░██
██░░░░░░██
██░░░░░░██
██████████
"
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 687);
    assert_eq!(
        part2(input),
        "\
████████░░░░████░░░░██░░░░██░░░░████░░░░██░░░░██░░██████░░░░████████░░░░████░░
██░░░░░░░░██░░░░██░░██░░██░░░░██░░░░██░░██░░██░░░░██░░░░██░░░░░░░░██░░██░░░░██
██████░░░░██░░░░░░░░████░░░░░░██░░░░░░░░████░░░░░░██████░░░░░░░░██░░░░██░░░░░░
██░░░░░░░░██░░████░░██░░██░░░░██░░░░░░░░██░░██░░░░██░░░░██░░░░██░░░░░░██░░████
██░░░░░░░░██░░░░██░░██░░██░░░░██░░░░██░░██░░██░░░░██░░░░██░░██░░░░░░░░██░░░░██
██░░░░░░░░░░██████░░██░░░░██░░░░████░░░░██░░░░██░░██████░░░░████████░░░░██████
"
    );
}
