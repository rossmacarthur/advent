use itertools::Itertools;

use crate::intcode::{cast, parse_program, State};

const INPUT: &str = include_str!("input/07.txt");

pub fn default_input() -> Vec<i64> {
    parse_program(INPUT)
}

#[derive(Debug)]
struct Computer {
    mem: Vec<i64>,
    ptr: usize,
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

fn make_computers(input: &[i64], phases: &[i64]) -> Vec<Computer> {
    phases
        .iter()
        .map(|&phase| {
            let mut computer = Computer::new(input.to_vec());
            assert!(matches!(computer.next(phase), State::Waiting));
            computer
        })
        .collect()
}

pub fn part1(input: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            let mut computers = make_computers(input, &phases);
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

pub fn part2(input: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut computers = make_computers(input, &phases);
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
