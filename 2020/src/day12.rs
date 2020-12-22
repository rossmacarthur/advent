use vector::i64::xy::{Vector, VectorExt};

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
                "N" => Move(Vector::new([0, value])),
                "S" => Move(Vector::new([0, -value])),
                "E" => Move(Vector::new([value, 0])),
                "W" => Move(Vector::new([-value, 0])),
                "L" => Turn(value),
                "R" => Turn(-value),
                "F" => Forward(value),
                op => panic!("unexpected operation `{}`", op),
            }
        })
        .collect()
}

pub fn part1(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::zero();
    let mut direction = Vector::new([1, 0]);
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
    ship.l1_norm()
}

pub fn part2(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::zero();
    let mut direction = Vector::new([10, 1]);
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
    ship.l1_norm()
}
