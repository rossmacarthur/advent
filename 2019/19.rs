mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/19.txt"))
}

fn is_affected(input: Vec<i64>, x: i64, y: i64) -> bool {
    let mut computer = Computer::new(input);
    computer.input(x);
    computer.input(y);
    computer.next().unwrap() == 1
}

fn part1(input: Vec<i64>) -> usize {
    (0..50)
        .cartesian_product(0..50)
        .filter(|(x, y)| is_affected(input.clone(), *x, *y))
        .count()
}

fn part2(input: Vec<i64>) -> i64 {
    const D: i64 = 99;
    let mut x = 0;
    let mut y = D;
    loop {
        while !is_affected(input.clone(), x, y) {
            x += 1;
        }
        // Check the other 3 corners are within the beam:
        //
        // a . . . b
        //         .
        //         .
        //         .
        // xy      c
        if [(x, y - D), (x + D, y - D), (x + D, y)]
            .iter()
            .copied()
            .all(|(x, y)| is_affected(input.clone(), x, y))
        {
            break x * 10_000 + (y - D);
        }
        y += 1;
    }
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 192);
    assert_eq!(part2(input), 8381082);
}
