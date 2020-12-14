use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day14.txt");

const DEFAULT_MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[derive(Debug)]
pub enum Instruction<'a> {
    Mask(&'a str),
    Assign(u64, u64),
}

use Instruction::*;

pub fn default_input() -> Vec<Instruction<'static>> {
    INPUT
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

fn from_bin(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

pub fn part1(instrs: &[Instruction]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = DEFAULT_MASK;
    for instr in instrs {
        match instr {
            Mask(m) => mask = m,
            Assign(address, value) => {
                let or = from_bin(&mask.replace('X', "0"));
                let and = from_bin(&mask.replace('X', "1"));
                let value = value | or & and;
                memory.insert(*address, value);
            }
        }
    }
    memory.values().sum()
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

pub fn part2(instrs: &[Instruction]) -> u64 {
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
