pub mod intcode;

use std::cmp::max;

use intcode::{cast, parse_program, State};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/09.txt"))
}

#[derive(Debug)]
pub struct Computer {
    mem: Vec<i64>,
    ptr: usize,
    relative_base: i64,
}

impl Computer {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            mem: program,
            ptr: 0,
            relative_base: 0,
        }
    }

    fn mem_get(&self, addr: usize) -> i64 {
        self.mem.get(addr).copied().unwrap_or(0)
    }

    fn mem_get_mut(&mut self, addr: usize) -> &mut i64 {
        self.mem.resize(max(self.mem.len(), addr + 1), 0);
        &mut self.mem[addr]
    }

    fn param_ptr(&self, i: usize) -> usize {
        let opcode = self.mem_get(self.ptr);
        let ptr = self.ptr + i;
        match opcode / (10i64.pow((1 + i) as u32)) % 10 {
            0 => cast(self.mem_get(ptr)),
            1 => ptr,
            2 => cast(self.relative_base + self.mem_get(ptr)),
            mode => panic!("unknown mode `{}`", mode),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem_get(self.param_ptr(i))
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        self.mem_get_mut(self.param_ptr(i))
    }

    pub fn next(&mut self, input: i64) -> State {
        let mut input = Some(input);
        loop {
            match self.mem_get(self.ptr) % 100 {
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
                9 => {
                    self.relative_base += self.param(1);
                    self.ptr += 2;
                }
                99 => break State::Complete,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }
}

fn part1(input: Vec<i64>) -> i64 {
    Computer::new(input).next(1).unwrap()
}

fn part2(input: Vec<i64>) -> i64 {
    Computer::new(input).next(2).unwrap()
}

fn main() {
    let mut run = advent::start();
    let input = run.time("Parse input", default_input());
    run.result("Part 1", part1(input.clone()));
    run.result("Part 2", part2(input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut computer = Computer::new(input.clone());
    let mut result = Vec::new();
    for _ in 0..input.len() {
        result.push(computer.next(1).unwrap());
    }
    assert_eq!(input, result);
}

#[test]
fn example2() {
    let input = parse_program("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(Computer::new(input).next(1).unwrap(), 1219070632396864);
}

#[test]
fn example3() {
    let input = parse_program("104,1125899906842624,99");
    assert_eq!(Computer::new(input).next(1).unwrap(), 1125899906842624)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2714716640);
    assert_eq!(part2(input), 58879);
}
