mod intcode;

use advent::prelude::*;
use intcode::{parse_program, State};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/07.txt"))
}

#[derive(Debug)]
struct Computer {
    mem: Vec<i64>,
    ptr: usize,
}

fn cast(num: i64) -> usize {
    usize::try_from(num).unwrap()
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            mem: program,
            ptr: 0,
        }
    }

    fn param_ptr(&self, i: usize) -> usize {
        let opcode = self.mem[self.ptr];
        let ptr = self.ptr + i;
        match opcode / (10i64.pow((1 + i) as u32)) % 10 {
            0 => cast(self.mem[ptr]),
            1 => ptr,
            mode => panic!("unknown mode `{}`", mode),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem[self.param_ptr(i)]
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        let ptr = self.param_ptr(i);
        &mut self.mem.as_mut_slice()[ptr]
    }

    fn next(&mut self, input: i64) -> State {
        let mut input = Some(input);
        loop {
            match self.mem[self.ptr] % 100 {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = input.take() {
                        *self.param_mut(1) = input;
                        self.ptr += 2;
                    } else {
                        break State::Waiting;
                    }
                }
                4 => {
                    let output = self.param(1);
                    self.ptr += 2;
                    break State::Yielded(output);
                }
                5 => {
                    if self.param(1) != 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if self.param(1) == 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    *self.param_mut(3) = (self.param(1) < self.param(2)) as i64;
                    self.ptr += 4;
                }
                8 => {
                    *self.param_mut(3) = (self.param(1) == self.param(2)) as i64;
                    self.ptr += 4;
                }
                99 => break State::Complete,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }
}

fn make_computers(input: Vec<i64>, phases: &[i64]) -> Vec<Computer> {
    phases
        .iter()
        .map(|&phase| {
            let mut computer = Computer::new(input.clone());
            assert!(matches!(computer.next(phase), State::Waiting));
            computer
        })
        .collect()
}

fn part1(input: Vec<i64>) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            let mut computers = make_computers(input.clone(), &phases);
            let mut signal = 0;
            for computer in computers.iter_mut() {
                signal = match computer.next(signal) {
                    State::Yielded(value) => value,
                    _ => panic!("unexpected state"),
                };
            }
            signal
        })
        .max()
        .unwrap()
}

fn part2(input: Vec<i64>) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut computers = make_computers(input.clone(), &phases);
            let mut signal = 0;
            loop {
                for computer in computers.iter_mut() {
                    signal = match computer.next(signal) {
                        State::Yielded(value) => value,
                        State::Complete => return signal,
                        State::Waiting => panic!("unexpected state"),
                    };
                }
            }
        })
        .max()
        .unwrap()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(part1(input), 43210);

    let input = parse_program(
        "3,23,3,24,1002,24,10,24,1002,23,-1,\
        23,101,5,23,23,1,24,23,23,4,23,99,0,0",
    );
    assert_eq!(part1(input), 54321);

    let input = parse_program(
        "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
         1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
    );
    assert_eq!(part1(input), 65210);
}

#[test]
fn example2() {
    let input = parse_program(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
         27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    assert_eq!(part2(input), 139629729);

    let input = parse_program(
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
         -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
         53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    );
    assert_eq!(part2(input), 18216);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 225056);
    assert_eq!(part2(input), 14260332);
}
