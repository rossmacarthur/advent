const INPUT: &str = include_str!("input/day12.txt");

pub enum Instruction {
    Move(i32, i32),
    Turn(i32),
    Forward(i32),
}

use Instruction::*;

pub fn default_input() -> Vec<Instruction> {
    INPUT
        .lines()
        .map(|line| {
            let (op, value) = line.split_at(1);
            let value = value.parse().unwrap();
            match op {
                "N" => Move(0, value),
                "S" => Move(0, -value),
                "E" => Move(value, 0),
                "W" => Move(-value, 0),
                "L" => Turn(value),
                "R" => Turn(-value),
                "F" => Forward(value),
                op => panic!("unexpected operation `{}`", op),
            }
        })
        .collect()
}

fn rotate((x, y): (i32, i32), angle: &i32) -> (i32, i32) {
    match angle.rem_euclid(360) {
        0 => (x, y),
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        angle => panic!("unexpected angle `{}`", angle),
    }
}

pub fn part1(instrs: &[Instruction]) -> i32 {
    let mut ship = (0, 0);
    let mut direction = (1, 0);
    for instr in instrs {
        match instr {
            Move(dx, dy) => {
                ship.0 += dx;
                ship.1 += dy;
            }
            Turn(angle) => {
                direction = rotate(direction, angle);
            }
            Forward(distance) => {
                ship.0 += direction.0 * distance;
                ship.1 += direction.1 * distance;
            }
        }
    }
    ship.0.abs() + ship.1.abs()
}

pub fn part2(instrs: &[Instruction]) -> i32 {
    let mut ship = (0, 0);
    let mut direction = (10, 1);
    for instr in instrs {
        match instr {
            Move(dx, dy) => {
                direction.0 += dx;
                direction.1 += dy;
            }
            Turn(angle) => {
                direction = rotate(direction, angle);
            }
            Forward(distance) => {
                ship.0 += direction.0 * distance;
                ship.1 += direction.1 * distance;
            }
        }
    }
    ship.0.abs() + ship.1.abs()
}
