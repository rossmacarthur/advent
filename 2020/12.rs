use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .split_whitespace()
        .map(|cmd| {
            let (op, v) = cmd.split_at(1);
            let v = v.parse().unwrap();
            match op {
                "N" => Instr::Move(vector![0, v]),
                "S" => Instr::Move(vector![0, -v]),
                "E" => Instr::Move(vector![v, 0]),
                "W" => Instr::Move(vector![-v, 0]),
                "L" => Instr::Turn(v),
                "R" => Instr::Turn(-v),
                "F" => Instr::Forward(v),
                op => panic!("unexpected operation `{op}`"),
            }
        })
        .collect()
}

fn default_input() -> Vec<Instr> {
    parse_input(include_str!("input/12.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Move(Vector2),
    Turn(i64),
    Forward(i64),
}

fn rotate(v: Vector2, a: i64) -> Vector2 {
    match a.rem_euclid(360) {
        0 => vector![v.x, v.y],
        90 => vector![-v.y, v.x],
        180 => vector![-v.x, -v.y],
        270 => vector![v.y, -v.x],
        a => panic!("unsupported oblique angle `{a}`"),
    }
}

fn part1(instrs: Vec<Instr>) -> i64 {
    let mut ship = Vector2::zero();
    let mut d = vector![1, 0];
    for instr in instrs {
        match instr {
            Instr::Move(v) => ship += v,
            Instr::Turn(a) => d = rotate(d, a),
            Instr::Forward(dist) => ship += d * dist,
        }
    }
    ship.l1_norm()
}

fn part2(instrs: Vec<Instr>) -> i64 {
    let mut ship = Vector2::zero();
    let mut d = vector![10, 1];
    for instr in instrs {
        match instr {
            Instr::Move(v) => d += v,
            Instr::Turn(a) => d = rotate(d, a),
            Instr::Forward(dist) => ship += d * dist,
        }
    }
    ship.l1_norm()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("F10 N3 F7 R90 F11");
    assert_eq!(part1(input.clone()), 25);
    assert_eq!(part2(input), 286);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 845);
    assert_eq!(part2(input), 27016);
}
