use std::array;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 15).trim()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Dash,
    Equals(u8),
}

/// Split a step into its label and operation.
///
/// E.g. "cm-"  -> ("cm", Op::Dash)
///      "rn=1" -> ("rn", Op::Equals(1))
fn split_op(step: &str) -> (&str, Op) {
    let s = step.as_bytes();
    let p = step.find(|c| matches!(c, '-' | '=')).unwrap();
    let label = &step[..p];
    let op = match &s[p] {
        b'-' => Op::Dash,
        b'=' => Op::Equals(s[p + 1] - b'0'),
        _ => unreachable!(),
    };
    (label, op)
}

/// Hash a label to a number between 0 and 255.
fn hash(label: &str) -> usize {
    label.bytes().fold(0, |h, b| (h + b as usize) * 17 % 256)
}

fn part1(init: &str) -> usize {
    init.split(',').map(hash).sum()
}

fn part2(init: &str) -> usize {
    let mut boxes: [Vec<_>; 256] = array::from_fn(|_| Vec::new());

    for step in init.split(',') {
        let (label, op) = split_op(step);
        let bx = &mut boxes[hash(label)];
        match op {
            Op::Dash => bx.retain(|&(l, _)| l != label),
            Op::Equals(lens) => match bx.iter().position(|&(l, _)| l == label) {
                Some(p) => bx[p] = (label, lens),
                None => bx.push((label, lens)),
            },
        }
    }

    let mut power = 0;
    for (b, slots) in boxes.iter().enumerate() {
        for (s, &(_, lens)) in slots.iter().enumerate() {
            power += (b + 1) * (s + 1) * (lens as usize);
        }
    }
    power
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
    assert_eq!(part2(input), 145);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 510273);
    assert_eq!(part2(input), 212449);
}
