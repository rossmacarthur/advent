use itertools::Itertools;

fn parse_input(s: &str) -> Vec<(Operation, i64)> {
    s.lines()
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

fn default_input() -> Vec<(Operation, i64)> {
    parse_input(include_str!("input/08.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

struct ProgramResult {
    is_finite: bool,
    instrs: Vec<usize>,
    acc: i64,
}

fn exec(program: &[(Operation, i64)]) -> ProgramResult {
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

fn part1(program: &[(Operation, i64)]) -> i64 {
    exec(program).acc
}

fn part2(program: &[(Operation, i64)]) -> i64 {
    let result = exec(program);
    for ptr in result.instrs {
        let new_op = match program[ptr].0 {
            Operation::Nop => Operation::Jmp,
            Operation::Jmp => Operation::Nop,
            _ => continue,
        };
        let mut new_program = program.to_vec();
        new_program[ptr].0 = new_op;
        let result = exec(&new_program);
        if result.is_finite {
            return result.acc;
        }
    }
    panic!("failed to fix infinite loop")
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
    let input = parse_input(
        r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#,
    );
    assert_eq!(part1(&input), 5);
    assert_eq!(part2(&input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 1584);
    assert_eq!(part2(&input), 920);
}
