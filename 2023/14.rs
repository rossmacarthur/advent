#![allow(clippy::needless_range_loop)]

use advent::prelude::*;

fn parse_input<const D: usize>(input: &str) -> Platform<D> {
    let mut platform = empty();
    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            platform[x][y] = match tile {
                '.' => Tile::Empty,
                'O' => Tile::Round,
                '#' => Tile::Cube,
                t => panic!("unexpected character `{t}`"),
            };
        }
    }
    platform
}

fn default_input() -> Platform<100> {
    parse_input(include_input!(2023 / 14))
}

type Platform<const D: usize> = [[Tile; D]; D];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Round,
    Cube,
}

/// Returns a new empty platform.
fn empty<const D: usize>() -> Platform<D> {
    [[Tile::Empty; D]; D]
}

/// Rotates the entire platform 90 degrees.
fn rotate<const D: usize>(platform: Platform<D>) -> Platform<D> {
    let mut rotated = empty();
    for x in 0..D {
        for y in 0..D {
            rotated[x][y] = platform[y][D - x - 1];
        }
    }
    rotated
}

/// Returns a new platform tilted such that all the round rocks fall to the top.
fn tilt<const D: usize>(mut platform: Platform<D>) -> Platform<D> {
    for col in &mut platform {
        for i in 0..D {
            if col[i] != Tile::Round {
                continue;
            }
            let mut j = i;
            while let Some(&Tile::Empty) = col.get(j - 1) {
                j -= 1;
            }
            col.swap(i, j);
        }
    }
    platform
}

/// Returns a new platform tilted in all four directions.
fn tilt4<const D: usize>(mut platform: Platform<D>) -> Platform<D> {
    for _ in 0..4 {
        platform = rotate(tilt(platform));
    }
    platform
}

/// Returns the load on the platform.
fn load<const D: usize>(platform: Platform<D>) -> usize {
    platform
        .into_iter()
        .flat_map(|col| {
            col.into_iter()
                .enumerate()
                .filter(|&(_, t)| t == Tile::Round)
                .map(|(i, _)| D - i)
        })
        .sum()
}

fn part1<const D: usize>(platform: Platform<D>) -> usize {
    load(tilt(platform))
}

fn part2<const D: usize>(mut platform: Platform<D>) -> usize {
    // first find where the cycle starts
    let start = {
        let mut i = 0;
        let mut seen = HashSet::new();
        while seen.insert(platform) {
            platform = tilt4(platform);
            i += 1;
        }
        i
    };

    // next find where the cycle ends
    let (platforms, len) = {
        let mut i = 0;
        let mut seen = Vec::new();
        while !seen.contains(&platform) {
            seen.push(platform);
            platform = tilt4(platform);
            i += 1;
        }
        (seen, i)
    };

    // finally, find the platform at the billionth iteration
    let platform = platforms[(1_000_000_000 - start) % len];
    load(platform)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input::<10>(
        "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    );
    assert_eq!(part1(input), 136);
    assert_eq!(part2(input), 64);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 108614);
    assert_eq!(part2(input), 96447);
}
