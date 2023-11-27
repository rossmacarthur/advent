mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/13.txt"))
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

enum Output {
    Tile(Vector2, Tile),
    Score(i64),
}

impl Computer {
    fn next_output(&mut self) -> Option<Output> {
        let x = self.next()?;
        let y = self.next()?;
        let t = self.next()?;
        Some(if x == -1 && y == 0 {
            Output::Score(t)
        } else {
            let tile = match t {
                0 => Tile::Empty,
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                t => panic!("invalid tile `{t}`"),
            };
            let pos = vector![x, y];
            Output::Tile(pos, tile)
        })
    }
}

fn part1(program: Vec<i64>) -> usize {
    let mut computer = Computer::new(program);
    iter::from_fn(|| computer.next_output())
        .filter(|o| matches!(o, Output::Tile(_, Tile::Block)))
        .count()
}

fn part2(mut program: Vec<i64>) -> i64 {
    program[0] = 2;

    let mut computer = Computer::new(program);
    let mut score = 0;
    let mut paddle = Vector2::zero();

    while let Some(output) = computer.next_output() {
        match output {
            Output::Tile(pos, Tile::Ball) => {
                computer.input((pos.x - paddle.x).signum());
            }
            Output::Tile(pos, Tile::Paddle) => paddle = pos,
            Output::Score(value) => score = value,
            _ => continue,
        }
    }

    score
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 398);
    assert_eq!(part2(input), 19447);
}
