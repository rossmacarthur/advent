mod intcode;

use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/09.txt"))
}

fn part1(input: Vec<i64>) -> i64 {
    Computer::new(input).input(1).next().unwrap()
}

fn part2(input: Vec<i64>) -> i64 {
    Computer::new(input).input(2).next().unwrap()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(input.clone()));
    run.part(|| part2(input.clone()));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut computer = Computer::new(input.clone());
    let mut result = Vec::new();
    for _ in 0..input.len() {
        result.push(computer.next().unwrap());
    }
    assert_eq!(input, result);
}

#[test]
fn example2() {
    let input = parse_program("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(Computer::new(input).next().unwrap(), 1219070632396864);
}

#[test]
fn example3() {
    let input = parse_program("104,1125899906842624,99");
    assert_eq!(Computer::new(input).next().unwrap(), 1125899906842624)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2714716640);
    assert_eq!(part2(input), 58879);
}
