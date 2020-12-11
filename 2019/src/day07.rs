use std::convert::TryFrom;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day07.txt");

pub fn default_input() -> Vec<i64> {
    INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn cast(num: i64) -> usize {
    usize::try_from(num).unwrap()
}

#[derive(Debug, PartialEq)]
enum State {
    Waiting,
    Yielded(i64),
    Complete,
}

#[derive(Debug)]
struct Computer {
    program: Vec<i64>,
    ptr: usize,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self { ptr: 0, program }
    }

    fn run(&mut self, input: i64) -> State {
        let program = self.program.as_mut_slice();
        let mut input = Some(input);
        loop {
            let opcode = program[self.ptr];

            macro_rules! param_ptr {
                ($i:expr) => {{
                    let ptr = self.ptr + $i;
                    match opcode / (10i64.pow(1 + $i)) % 10 {
                        0 => cast(program[ptr]),
                        1 => ptr,
                        mode => panic!("unknown mode `{}`", mode),
                    }
                }};
            }
            macro_rules! param {
                ($i:expr) => {
                    program[param_ptr!($i)]
                };
            }
            macro_rules! param_mut {
                ($i:expr) => {
                    &mut program[param_ptr!($i)]
                };
            }

            match opcode % 100 {
                1 => {
                    *param_mut!(3) = param!(1) + param!(2);
                    self.ptr += 4;
                }
                2 => {
                    *param_mut!(3) = param!(1) * param!(2);
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = input.take() {
                        *param_mut!(1) = input;
                        self.ptr += 2;
                    } else {
                        break State::Waiting;
                    }
                }
                4 => {
                    let output = param!(1);
                    self.ptr += 2;
                    break State::Yielded(output);
                }
                5 => {
                    if param!(1) != 0 {
                        self.ptr = cast(param!(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if param!(1) == 0 {
                        self.ptr = cast(param!(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    *param_mut!(3) = (param!(1) < param!(2)) as i64;
                    self.ptr += 4;
                }
                8 => {
                    *param_mut!(3) = (param!(1) == param!(2)) as i64;
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
            assert_eq!(computer.run(phase), State::Waiting);
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
                signal = match computer.run(signal) {
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
                    signal = match computer.run(signal) {
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
