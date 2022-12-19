mod intcode;

use intcode::parse_program;

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/05.txt"))
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
            mode => panic!("unknown mode `{mode}`"),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem[self.param_ptr(i)]
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        let ptr = self.param_ptr(i);
        &mut self.mem.as_mut_slice()[ptr]
    }

    fn run(&mut self, input: i64) -> i64 {
        let mut output = None;
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
                    *self.param_mut(1) = input;
                    self.ptr += 2;
                }
                4 => {
                    output = Some(self.param(1));
                    self.ptr += 2;
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
                99 => break,
                opcode => panic!("unknown opcode `{opcode}`"),
            }
        }
        output.expect("program has no output")
    }
}

fn part1(input: Vec<i64>) -> i64 {
    Computer::new(input).run(1)
}

fn part2(input: Vec<i64>) -> i64 {
    Computer::new(input).run(5)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_program(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    );
    assert_eq!(Computer::new(input.clone()).run(7), 999);
    assert_eq!(Computer::new(input.clone()).run(8), 1000);
    assert_eq!(Computer::new(input).run(9), 1001);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 11933517);
    assert_eq!(part2(input), 10428568);
}
