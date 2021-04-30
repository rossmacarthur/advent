use std::collections::HashMap;

use itertools::Itertools;

use Instruction::*;

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.split(" = ").next_tuple().unwrap() {
            ("mask", mask) => Mask(mask),
            (left, right) => {
                let address = left
                    .trim_start_matches("mem[")
                    .trim_end_matches(']')
                    .parse()
                    .unwrap();
                let value = right.parse().unwrap();
                Assign(address, value)
            }
        })
        .collect()
}

fn default_input() -> Vec<Instruction<'static>> {
    parse_input(include_str!("input/14.txt"))
}

const DEFAULT_MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Assign(u64, u64),
}

fn from_bin(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn combinations(address: u64, mask: &str, i: usize) -> Vec<u64> {
    if i == mask.len() {
        return vec![0];
    }
    let curr = mask.chars().nth(mask.len() - i - 1).unwrap();
    combinations(address, mask, i + 1)
        .into_iter()
        .flat_map(|n| match curr {
            '0' => vec![n | (address & (1 << i))],
            '1' => vec![n | 1 << i],
            'X' => vec![n, n | (1 << i)],
            c => panic!("unexpected mask value `{}`", c),
        })
        .collect()
}

fn part1(instrs: &[Instruction]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = DEFAULT_MASK;
    for instr in instrs {
        match instr {
            Mask(m) => mask = m,
            Assign(address, value) => {
                let or = from_bin(&mask.replace('X', "0"));
                let and = from_bin(&mask.replace('X', "1"));
                let value = (value | or) & and;
                memory.insert(*address, value);
            }
        }
    }
    memory.values().sum()
}

fn part2(instrs: &[Instruction]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = DEFAULT_MASK;
    for instr in instrs {
        match instr {
            Mask(m) => mask = m,
            Assign(address, value) => {
                for address in combinations(*address, mask, 0) {
                    memory.insert(address, *value);
                }
            }
        }
    }
    memory.values().sum()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
    );
    assert_eq!(part1(&input), 165);
}

#[test]
fn example2() {
    let input = parse_input(
        "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
    );
    assert_eq!(part2(&input), 208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 11884151942312);
    assert_eq!(part2(&input), 2625449018811);
}
