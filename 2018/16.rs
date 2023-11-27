mod device;

use advent::prelude::*;
use device::{compute, Op};

fn parse_nums(s: &str) -> [usize; 4] {
    s.split_whitespace()
        .map(|s| s.trim_end_matches(',').parse().unwrap())
        .next_array()
        .unwrap()
}

fn parse_sample(s: &str) -> Sample {
    let [before, instr, after] = s.lines().next_array().unwrap();
    Sample {
        before: parse_nums(before.trim_start_matches("Before: [").trim_end_matches(']')),
        instr: parse_nums(instr),
        after: parse_nums(after.trim_start_matches("After:  [").trim_end_matches(']')),
    }
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<[usize; 4]>) {
    let [samples, prog] = input.split("\n\n\n\n").next_array().unwrap();
    let samples = samples.split("\n\n").map(parse_sample).collect();
    let prog = prog.lines().map(parse_nums).collect();
    (samples, prog)
}

fn default_input() -> (Vec<Sample>, Vec<[usize; 4]>) {
    parse_input(include_str!("input/16.txt"))
}

#[derive(Debug, Clone)]
struct Sample {
    before: [usize; 4],
    instr: [usize; 4],
    after: [usize; 4],
}

impl Op {
    #[rustfmt::skip]
    fn iter() -> impl Iterator<Item = Self> {
        [
            Op::Addr, Op::Addi,
            Op::Mulr, Op::Muli,
            Op::Banr, Op::Bani,
            Op::Borr, Op::Bori,
            Op::Setr, Op::Seti,
            Op::Gtir, Op::Gtri, Op::Gtrr,
            Op::Eqir, Op::Eqri, Op::Eqrr,
        ]
        .into_iter()
    }
}

fn part1((samples, _): (Vec<Sample>, Vec<[usize; 4]>)) -> usize {
    samples
        .iter()
        .filter(|s| {
            let [_, a, b, c] = s.instr;
            let n = Op::iter()
                .filter(move |&op| compute(s.before, (op, [a, b, c])) == s.after)
                .count();
            n >= 3
        })
        .count()
}

fn part2((samples, prog): (Vec<Sample>, Vec<[usize; 4]>)) -> usize {
    // First find all the opcodes that each value could possibly be.
    let mut possible: HashMap<_, HashSet<_>> = samples
        .iter()
        .map(|s| {
            let [o, a, b, c] = s.instr;
            let ops = Op::iter().filter(move |&op| compute(s.before, (op, [a, b, c])) == s.after);
            (o, ops)
        })
        .fold(HashMap::new(), |mut acc, (o, ops)| {
            acc.entry(o).or_default().extend(ops);
            acc
        });

    // Now we solve be finding a number that can only be one opcode, and
    // removing that opcode as a possibility for others.
    let mut resolved = HashMap::new();
    while !possible.is_empty() {
        let ps: Vec<_> = possible
            .iter()
            .filter(|(_, ops)| ops.len() == 1)
            .map(|(o, ops)| (*o, *ops.iter().next().unwrap()))
            .collect();
        for (o, op) in ps {
            possible.remove(&o);
            resolved.insert(o, op);
            for others in possible.values_mut() {
                others.remove(&op);
            }
        }
    }

    // Simply compute the program using the resolved opcode mapping.
    let mut regs = [0; 4];
    for [o, a, b, c] in prog {
        let op = resolved[&o];
        regs = compute(regs, (op, [a, b, c]));
    }
    regs[0]
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let s = parse_sample(
        "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]",
    );
    let [_, a, b, c] = s.instr;

    // behaves like these opcodes
    assert_eq!(compute(s.before, (Op::Addi, [a, b, c])), s.after);
    assert_eq!(compute(s.before, (Op::Mulr, [a, b, c])), s.after);
    assert_eq!(compute(s.before, (Op::Seti, [a, b, c])), s.after);

    // does not behave like the rest
    assert_ne!(compute(s.before, (Op::Addr, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Muli, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Banr, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Bani, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Borr, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Bori, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Setr, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Gtir, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Gtri, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Gtrr, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Eqir, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Eqri, [a, b, c])), s.after);
    assert_ne!(compute(s.before, (Op::Eqrr, [a, b, c])), s.after);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 636);
    assert_eq!(part2(input), 674);
}
