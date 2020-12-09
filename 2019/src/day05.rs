use std::convert::TryFrom;

const INPUT: &str = include_str!("input/day05.txt");

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

fn exec_intcode(mut program: Vec<i64>, input: i64) -> Option<i64> {
    let program = program.as_mut_slice();
    let mut ptr: usize = 0;
    let mut output = None;
    loop {
        let opcode = program[ptr];

        macro_rules! param_ptr {
            ($i:expr) => {{
                let ptr = ptr + $i;
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
                ptr += 4;
            }
            2 => {
                *param_mut!(3) = param!(1) * param!(2);
                ptr += 4;
            }
            3 => {
                *param_mut!(1) = input;
                ptr += 2;
            }
            4 => {
                output = Some(param!(1));
                ptr += 2;
            }
            5 => {
                if param!(1) != 0 {
                    ptr = cast(param!(2));
                } else {
                    ptr += 3;
                }
            }
            6 => {
                if param!(1) == 0 {
                    ptr = cast(param!(2));
                } else {
                    ptr += 3;
                }
            }
            7 => {
                *param_mut!(3) = (param!(1) < param!(2)) as i64;
                ptr += 4;
            }
            8 => {
                *param_mut!(3) = (param!(1) == param!(2)) as i64;
                ptr += 4;
            }
            99 => break,
            opcode => panic!("unknown opcode `{}`", opcode),
        }
    }
    output
}

pub fn part1(input: &[i64]) -> i64 {
    exec_intcode(input.to_vec(), 1).unwrap()
}

pub fn part2(input: &[i64]) -> i64 {
    exec_intcode(input.to_vec(), 5).unwrap()
}
