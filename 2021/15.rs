use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use vectrix::{parse_map, vector, Vector2, CARDINALS};

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> HashMap<Vector, i64> {
    parse_map(input, |c| match c {
        c @ '0'..='9' => c as i64 - '0' as i64,
        c => panic!("unexpected character `{}`", c),
    })
}

fn default_input() -> HashMap<Vector, i64> {
    parse_input(include_str!("input/15.txt"))
}

fn solve(map: HashMap<Vector, i64>) -> i64 {
    let start = vector![0, 0];
    let end = vector![
        map.keys().map(|p| p.x).max().unwrap(),
        map.keys().map(|p| p.y).max().unwrap()
    ];

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push((Reverse(0), start));
    while let Some((Reverse(risk), p)) = pq.pop() {
        if !visited.insert(p) {
            continue;
        }
        for d in CARDINALS {
            let next = p + d;
            if let Some(r) = map.get(&next) {
                if next == end {
                    return risk + r;
                }
                pq.push((Reverse(risk + r), next));
            }
        }
    }
    panic!("no path found")
}

fn part1(input: HashMap<Vector, i64>) -> i64 {
    solve(input)
}

fn part2(input: HashMap<Vector, i64>) -> i64 {
    let w = input.keys().map(|p| p.x).max().unwrap() + 1;
    let h = input.keys().map(|p| p.y).max().unwrap() + 1;

    let map = (0..5)
        .cartesian_product(0..5)
        .flat_map(|(i, j)| {
            input.iter().map(move |(p, r)| {
                let x = p.x + w * i;
                let y = p.y + h * j;
                let risk = ((r + i + j) - 1) % 9 + 1;
                (vector![x, y], risk)
            })
        })
        .collect();

    solve(map)
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
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    );
    assert_eq!(part1(input.clone()), 40);
    assert_eq!(part2(input), 315);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 366);
    assert_eq!(part2(input), 2829);
}
