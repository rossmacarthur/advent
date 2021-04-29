mod intcode;

use intcode::parse_program;

fn default_input() -> Vec<usize> {
    parse_program(include_str!("input/02.txt"))
}

#[derive(Debug)]
struct Computer {
    mem: Vec<usize>,
    ptr: usize,
}

impl Computer {
    fn new(program: Vec<usize>) -> Self {
        Self {
            mem: program,
            ptr: 0,
        }
    }

    fn param(&self, i: usize) -> usize {
        let ptr = self.mem[self.ptr + i];
        self.mem[ptr]
    }

    fn param_mut(&mut self, i: usize) -> &mut usize {
        let ptr = self.mem[self.ptr + i];
        &mut self.mem[ptr]
    }

    fn run(&mut self) {
        loop {
            match self.mem[self.ptr] {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                99 => break,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }
}

fn run(input: &[usize], noun: usize, verb: usize) -> usize {
    let mut computer = Computer::new(input.to_vec());
    computer.mem[1] = noun;
    computer.mem[2] = verb;
    computer.run();
    computer.mem[0]
}

fn part1(input: &[usize]) -> usize {
    run(input, 12, 2)
}

fn part2(input: &[usize]) -> usize {
    for noun in 0..input.len() {
        for verb in 0..input.len() {
            if run(input, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no valid noun and verb found")
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
    let mut computer = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    computer.run();
    assert_eq!(computer.mem[0], 3500);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 3850704);
    assert_eq!(part2(&input), 6718);
}
