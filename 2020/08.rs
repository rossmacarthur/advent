use advent::prelude::*;

fn parse_input(s: &str) -> Vec<(Op, i64)> {
    s.lines()
        .map(|line| {
            let [op, arg] = line.split_whitespace().next_array().unwrap();
            let op = match op {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                op => panic!("unknown operation `{op}`"),
            };
            (op, arg.parse().unwrap())
        })
        .collect()
}

fn default_input() -> Vec<(Op, i64)> {
    parse_input(include_str!("input/08.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
struct Result {
    is_finite: bool,
    instrs: Vec<usize>,
    acc: i64,
}

fn exec(prog: &[(Op, i64)]) -> Result {
    let mut instrs = Vec::new();
    let mut ptr = 0;
    let mut acc = 0;
    while !instrs.contains(&ptr) && ptr < prog.len() {
        instrs.push(ptr);
        match prog[ptr] {
            (Op::Nop, _) => {
                ptr += 1;
            }
            (Op::Acc, inc) => {
                ptr += 1;
                acc += inc;
            }
            (Op::Jmp, offset) => {
                ptr = (ptr as i64 + offset) as usize;
            }
        }
    }
    Result {
        is_finite: ptr == prog.len(),
        instrs,
        acc,
    }
}

fn part1(prog: Vec<(Op, i64)>) -> i64 {
    exec(&prog).acc
}

fn part2(mut prog: Vec<(Op, i64)>) -> i64 {
    for ptr in exec(&prog).instrs {
        let old = prog[ptr].0;
        let new = match old {
            Op::Nop => Op::Jmp,
            Op::Jmp => Op::Nop,
            _ => continue,
        };
        prog[ptr].0 = new;
        let Result { is_finite, acc, .. } = exec(&prog);
        if is_finite {
            return acc;
        }
        prog[ptr].0 = old;
    }
    panic!("failed to fix infinite loop")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
    );
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1584);
    assert_eq!(part2(input), 920);
}
