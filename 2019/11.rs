#[path = "09.rs"]
#[allow(dead_code)]
mod bin09;

use std::collections::BTreeMap;

use vectrix::{vector, Vector2, VectorExt};

use bin09::intcode::{parse_program, State};
use bin09::Computer;

type Vector = Vector2<i64>;

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/11.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

use Color::*;
use Turn::*;

impl Computer {
    fn next_turn(&mut self, input: i64) -> Turn {
        match self.next(input).unwrap() {
            0 => Left,
            1 => Right,
            turn => panic!("invalid turn `{}`", turn),
        }
    }

    fn next_color(&mut self, input: i64) -> Option<Color> {
        match self.next(input) {
            State::Complete => None,
            State::Yielded(0) => Some(Black),
            State::Yielded(1) => Some(White),
            state => panic!("unexpected state `{:?}`", state),
        }
    }

    fn next_color_and_turn(&mut self, color: Color) -> Option<(Color, Turn)> {
        let input = color as i64;
        self.next_color(input)
            .map(|color| (color, self.next_turn(input)))
    }
}

fn rotate(vector: Vector, turn: Turn) -> Vector {
    match turn {
        Left => vector.rotated(90),
        Right => vector.rotated(-90),
    }
}

fn paint(program: Vec<i64>, color: Color) -> BTreeMap<Vector, Color> {
    let mut computer = Computer::new(program);
    let mut map = BTreeMap::new();
    let mut position = Vector::zero();
    let mut direction = vector![0, 1];
    while let Some((color, turn)) =
        computer.next_color_and_turn(*map.get(&position).unwrap_or(&color))
    {
        map.insert(position, color);
        direction = rotate(direction, turn);
        position += direction;
    }
    map
}

fn part1(program: Vec<i64>) -> usize {
    paint(program, Black).len()
}

fn part2(program: Vec<i64>) -> String {
    let map = paint(program, White);

    let min_x = map.keys().map(|v| v.x).min().unwrap();
    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let min_y = map.keys().map(|v| v.y).min().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    let mut result = String::from('\n');
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push_str(match map.get(&vector![x, y]) {
                Some(Black) | None => "  ",
                Some(White) => "██",
            })
        }
        result.push('\n');
    }
    result
}

fn main() {
    let mut run = advent::start();
    let input = run.time("Parse input", default_input());
    run.result("Part 1", part1(input.clone()));
    run.result("Part 2", part2(input));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1930);
}
