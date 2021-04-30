use vectrix::{vector, Vector2, VectorExt};

use Instruction::*;

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split_whitespace()
        .map(|cmd| {
            let (op, value) = cmd.split_at(1);
            let value = value.parse().unwrap();
            match op {
                "N" => Move(vector![0, value]),
                "S" => Move(vector![0, -value]),
                "E" => Move(vector![value, 0]),
                "W" => Move(vector![-value, 0]),
                "L" => Turn(value),
                "R" => Turn(-value),
                "F" => Forward(value),
                op => panic!("unexpected operation `{}`", op),
            }
        })
        .collect()
}

fn default_input() -> Vec<Instruction> {
    parse_input(include_str!("input/12.txt"))
}

enum Instruction {
    Move(Vector),
    Turn(i64),
    Forward(i64),
}

fn part1(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::zero();
    let mut direction = vector![1, 0];
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

fn part2(instrs: &[Instruction]) -> i64 {
    let mut ship = Vector::zero();
    let mut direction = vector![10, 1];
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

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("F10 N3 F7 R90 F11");
    assert_eq!(part1(&input), 25);
    assert_eq!(part2(&input), 286);
}


#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 845);
    assert_eq!(part2(&input), 27016);
}
