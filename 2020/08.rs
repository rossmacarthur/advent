use itertools::Itertools;

const INPUT: &str = include_str!("input/08.txt");

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Nop,
    Acc,
    Jmp,
}

struct ProgramResult {
    is_finite: bool,
    instrs: Vec<usize>,
    acc: i64,
}

pub fn default_input() -> Vec<(Operation, i64)> {
    INPUT
        .lines()
        .map(|line| {
            let (op, arg) = line.split_whitespace().next_tuple().unwrap();
            (
                match op {
                    "nop" => Operation::Nop,
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    op => panic!("unknown operation `{}`", op),
                },
                arg.parse().unwrap(),
            )
        })
        .collect()
}

fn exec_boot_code(program: &[(Operation, i64)]) -> ProgramResult {
    let mut instrs = Vec::new();
    let mut acc = 0;
    let mut ptr = 0;
    while !instrs.contains(&ptr) && ptr < program.len() {
        instrs.push(ptr);
        let (op, arg) = program[ptr];
        match op {
            Operation::Nop => {
                ptr += 1;
            }
            Operation::Acc => {
                acc += arg;
                ptr += 1;
            }
            Operation::Jmp => {
                ptr = (ptr as i64 + arg) as usize;
            }
        }
    }
    ProgramResult {
        is_finite: ptr == program.len(),
        instrs,
        acc,
    }
}

pub fn part1(program: &[(Operation, i64)]) -> i64 {
    exec_boot_code(program).acc
}

pub fn part2(program: &[(Operation, i64)]) -> i64 {
    let result = exec_boot_code(program);
    for ptr in result.instrs {
        let new_op = match program[ptr].0 {
            Operation::Nop => Operation::Jmp,
            Operation::Jmp => Operation::Nop,
            _ => continue,
        };
        let mut new_program = program.to_vec();
        new_program[ptr].0 = new_op;
        let result = exec_boot_code(&new_program);
        if result.is_finite {
            return result.acc;
        }
    }
    panic!("failed to fix infinite loop")
}
