mod intcode;

use std::collections::HashMap;

use vectrix::{vector, Vector};

use intcode::{parse_program, Computer, State};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/19.txt"))
}

fn part1(input: Vec<i64>) -> usize {
    // let mut computer = Computer::new(input);

    let mut map = HashMap::new();
    for x in 0..190 {
        for y in 0..190 {
            let mut computer = Computer::new(input.clone());
            computer.input(vec![x, y]);
            let c = match computer.next() {
                State::Yielded(0) => '.',
                State::Yielded(1) => '#',
                State::Complete => break,
                i => panic!("unexpected response `{:?}`", i),
            };
            map.insert(vector![x, y], c);
        }
    }

    let min_x = map.keys().map(|v| v.x).min().unwrap();
    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let min_y = map.keys().map(|v| v.y).min().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    let mut result = String::from('\n');
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push_str(match map.get(&vector![x, y]) {
                Some('.') | None => ".",
                Some('#') => "#",
                _ => todo!(),
            })
        }
        result.push('\n');
    }
    println!("{}", result);

    map.values().filter(|c| matches!(c, '#')).count()
}

fn part2(input: &[i64]) -> i64 {
    todo!()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(input.clone()));
    // run.part(|| part2(&input));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 1);
    assert_eq!(part2(&input), 2);
}
