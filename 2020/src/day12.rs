use vector::Vector;

const INPUT: &str = include_str!("input/day12.txt");

pub enum Instruction {
    Move(Vector),
    Turn(i64),
    Forward(i64),
}

use Instruction::*;

pub fn default_input() -> Vec<Instruction> {
    INPUT
        .lines()
        .map(|line| {
            let (op, value) = line.split_at(1);
            let value = value.parse().unwrap();
            match op {
                "N" => Move(Vector::two(0, value)),
                "S" => Move(Vector::two(0, -value)),
                "E" => Move(Vector::two(value, 0)),
                "W" => Move(Vector::two(-value, 0)),
                "L" => Turn(value),
                "R" => Turn(-value),
                "F" => Forward(value),
                op => panic!("unexpected operation `{}`", op),
            }
        })
        .collect()
}

pub fn part1(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::two(0, 0);
    let mut direction = Vector::two(1, 0);
    for instr in instrs {
        match instr {
            Move(vector) => {
                ship += vector;
            }
            Turn(angle) => {
                direction = direction.rotated(*angle);
            }
            Forward(distance) => {
                ship += direction * distance;
            }
        }
    }
    ship.manhattan_distance()
}

pub fn part2(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::two(0, 0);
    let mut direction = Vector::two(10, 1);
    for instr in instrs {
        match instr {
            Move(vector) => {
                direction += vector;
            }
            Turn(angle) => {
                direction = direction.rotated(*angle);
            }
            Forward(distance) => {
                ship += direction * distance;
            }
        }
    }
    ship.manhattan_distance()
}
