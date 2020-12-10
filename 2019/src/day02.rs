const INPUT: &str = include_str!("input/day02.txt");

pub fn default_input() -> Vec<usize> {
    INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn exec_basic<F>(program: &mut Vec<usize>, ptr: usize, f: F) -> usize
where
    F: FnOnce(usize, usize) -> usize,
{
    let x_ptr = program[ptr + 1];
    let y_ptr = program[ptr + 2];
    let z_ptr = program[ptr + 3];
    program[z_ptr] = f(program[x_ptr], program[y_ptr]);
    ptr + 4
}

fn exec_intcode(mut program: &mut Vec<usize>) {
    let mut ptr = 0;
    loop {
        match program[ptr] {
            1 => ptr = exec_basic(&mut program, ptr, |x, y| x + y),
            2 => ptr = exec_basic(&mut program, ptr, |x, y| x * y),
            99 => break,
            opcode => panic!("unknown opcode `{}`", opcode),
        }
    }
}

pub fn part1(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    program[1] = 12;
    program[2] = 2;
    exec_intcode(&mut program);
    program[0]
}

pub fn part2(input: &[usize]) -> Option<usize> {
    for noun in 0..input.len() {
        for verb in 0..input.len() {
            let mut program = input.to_vec();
            program[1] = noun;
            program[2] = verb;
            exec_intcode(&mut program);
            if program[0] == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}
