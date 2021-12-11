use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

use vectrix::{parse_map, Vector2, CARDINALS_8};

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> HashMap<Vector, i64> {
    parse_map(input, |c| match c {
        c @ '0'..='9' => c as i64 - '0' as i64,
        c => panic!("unexpected character `{}`", c),
    })
}

fn default_input() -> HashMap<Vector, i64> {
    parse_input(include_str!("input/11.txt"))
}

fn step(map: &mut HashMap<Vector, i64>) -> usize {
    // Increase all energy levels
    for e in map.values_mut() {
        *e += 1;
    }

    // Handle flashes
    let mut q: VecDeque<_> = map
        .iter()
        .filter_map(|(&p, &e)| (e > 9).then(|| p))
        .collect();
    let mut flashed = HashSet::new();
    while let Some(p) = q.pop_front() {
        if flashed.contains(&p) {
            continue;
        }
        flashed.insert(p);
        for d in CARDINALS_8 {
            let next = p + d;
            if let Some(e) = map.get_mut(&next) {
                *e += 1;
                if *e > 9 {
                    q.push_back(next)
                }
            }
        }
    }

    // Reset all energy levels
    for e in map.values_mut() {
        if *e > 9 {
            *e = 0;
        }
    }

    flashed.len()
}

fn part1(mut map: HashMap<Vector, i64>) -> usize {
    iter::repeat_with(|| step(&mut map)).take(100).sum()
}

fn part2(mut map: HashMap<Vector, i64>) -> usize {
    let limit = map.len();
    1 + iter::repeat_with(|| step(&mut map))
        .position(|n| n == limit)
        .unwrap()
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
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    assert_eq!(part1(input.clone()), 1656);
    assert_eq!(part2(input), 195);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1793);
    assert_eq!(part2(input), 247);
}
