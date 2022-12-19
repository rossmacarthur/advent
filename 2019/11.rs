mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

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

impl Computer {
    fn next_color_and_turn(&mut self, color: Color) -> Option<(Color, Turn)> {
        self.input(color as i64);
        let color = self.next().map(|v| match v {
            0 => Color::Black,
            1 => Color::White,
            c => panic!("invalid color `{c}`"),
        })?;
        let turn = self.next().map(|v| match v {
            0 => Turn::Left,
            1 => Turn::Right,
            t => panic!("invalid turn `{t}`"),
        })?;
        Some((color, turn))
    }
}

fn paint(program: Vec<i64>, color: Color) -> BTreeMap<Vector2, Color> {
    let mut computer = Computer::new(program);
    let mut map = BTreeMap::new();
    let mut p = Vector2::zero();
    let mut d = vector![0, 1];
    while let Some((color, turn)) = computer.next_color_and_turn(*map.get(&p).unwrap_or(&color)) {
        map.insert(p, color);
        d = match turn {
            Turn::Left => vector![-d.y, d.x],
            Turn::Right => vector![d.y, -d.x],
        };
        p += d;
    }
    map
}

fn part1(program: Vec<i64>) -> usize {
    paint(program, Color::Black).len()
}

fn part2(program: Vec<i64>) -> String {
    let map = paint(program, Color::White);

    let min_x = map.keys().map(|v| v.x).min().unwrap();
    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let min_y = map.keys().map(|v| v.y).min().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    let mut result = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result += match map.get(&vector![x, y]) {
                Some(Color::Black) | None => "░░",
                Some(Color::White) => "██",
            };
        }
        result += "\n"
    }
    result
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1930);
    assert_eq!(
        part2(input),
        "\
░░██████░░░░████████░░██░░░░██░░██░░░░██░░████████░░░░████░░░░████████░░██░░░░██░░░░░░
░░██░░░░██░░██░░░░░░░░██░░██░░░░██░░░░██░░██░░░░░░░░██░░░░██░░░░░░░░██░░██░░░░██░░░░░░
░░██░░░░██░░██████░░░░████░░░░░░████████░░██████░░░░██░░░░░░░░░░░░██░░░░██░░░░██░░░░░░
░░██████░░░░██░░░░░░░░██░░██░░░░██░░░░██░░██░░░░░░░░██░░░░░░░░░░██░░░░░░██░░░░██░░░░░░
░░██░░░░░░░░██░░░░░░░░██░░██░░░░██░░░░██░░██░░░░░░░░██░░░░██░░██░░░░░░░░██░░░░██░░░░░░
░░██░░░░░░░░██░░░░░░░░██░░░░██░░██░░░░██░░████████░░░░████░░░░████████░░░░████░░░░░░░░
"
    )
}
