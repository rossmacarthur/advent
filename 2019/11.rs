mod intcode;

use std::collections::BTreeMap;

use vectrix::{vector, Vector2, VectorExt};

use intcode::{parse_program, Computer};

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
    fn next_color_and_turn(&mut self, color: Color) -> Option<(Color, Turn)> {
        self.input(color as i64);
        let color = self.next_value().map(|v| match v {
            0 => Black,
            1 => White,
            c => panic!("invalid color `{}`", c),
        })?;
        let turn = self.next_value().map(|v| match v {
            0 => Left,
            1 => Right,
            t => panic!("invalid turn `{}`", t),
        })?;
        Some((color, turn))
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
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(input.clone()));
    run.part(|| part2(input.clone()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1930);
}
