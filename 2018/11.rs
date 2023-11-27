use std::fmt;

use advent::prelude::*;

const fn default_input() -> i64 {
    9810
}

#[derive(Debug, PartialEq, Eq)]
pub struct Show<T>(pub T);

impl<T: fmt::Debug> fmt::Display for Show<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

fn power(serial: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let p = (rack_id * y + serial) * rack_id;
    (p / 100) % 10 - 5
}

// See https://en.wikipedia.org/wiki/Summed-area_table
fn solve(serial: i64, sizes: impl Iterator<Item = usize>) -> [usize; 3] {
    let mut grid = vec![vec![0; 301]; 301];
    for y in 1..=300 {
        for x in 1..=300 {
            let p = power(serial, x as i64, y as i64);
            grid[y][x] = p + grid[y - 1][x] + grid[y][x - 1] - grid[y - 1][x - 1];
        }
    }

    let mut max_s = 0;
    let mut max_y = 0;
    let mut max_x = 0;
    let mut max_p = 0;

    for s in sizes {
        for y in s..=300 {
            for x in s..=300 {
                let power = grid[y][x] - grid[y - s][x] - grid[y][x - s] + grid[y - s][x - s];
                if power > max_p {
                    max_s = s;
                    max_y = y;
                    max_x = x;
                    max_p = power;
                }
            }
        }
    }

    [max_x - max_s + 1, max_y - max_s + 1, max_s]
}

fn part1(serial: i64) -> Show<[usize; 2]> {
    let [x, y, _] = solve(serial, iter::once(3));
    Show([x, y])
}

fn part2(serial: i64) -> Show<[usize; 3]> {
    Show(solve(serial, 1..=300))
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    for (input, exp1, exp2) in [
        (18, [33, 45], [90, 269, 16]),
        (42, [21, 61], [232, 251, 12]),
    ] {
        assert_eq!(part1(input), Show(exp1));
        assert_eq!(part2(input), Show(exp2));
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), Show([245, 14]));
    assert_eq!(part2(input), Show([235, 206, 13]));
}
