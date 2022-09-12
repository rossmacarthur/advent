#![allow(dead_code)]

use advent::prelude::*;

pub type Instr = (Op, [usize; 3]);

#[derive(Clone)]
pub struct Program {
    pub ip: usize,
    pub instrs: Vec<(Op, [usize; 3])>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn parse_op(input: &str) -> Op {
    match input {
        "addr" => Op::Addr,
        "addi" => Op::Addi,
        "mulr" => Op::Mulr,
        "muli" => Op::Muli,
        "banr" => Op::Banr,
        "bani" => Op::Bani,
        "borr" => Op::Borr,
        "bori" => Op::Bori,
        "setr" => Op::Setr,
        "seti" => Op::Seti,
        "gtir" => Op::Gtir,
        "gtri" => Op::Gtri,
        "gtrr" => Op::Gtrr,
        "eqir" => Op::Eqir,
        "eqri" => Op::Eqri,
        "eqrr" => Op::Eqrr,
        i => panic!("unexpected instruction `{}`", i),
    }
}

fn parse_instr(input: &str) -> Instr {
    let mut it = input.split_whitespace();
    let op = it.next().map(parse_op).unwrap();
    let args = it.map(str::parse).map(Result::unwrap).next_array().unwrap();
    (op, args)
}

pub fn parse_program(input: &str) -> Program {
    let (ip, instrs) = input.split_once('\n').unwrap();
    let ip = ip.trim_start_matches("#ip ").parse().unwrap();
    let instrs = instrs.lines().map(parse_instr).collect();
    Program { ip, instrs }
}

pub fn compute<const N: usize>(mut regs: [usize; N], instr: Instr) -> [usize; N] {
    let (op, [a, b, c]) = instr;
    let res = match op {
        Op::Addr => regs[a] + regs[b],
        Op::Addi => regs[a] + b,
        Op::Mulr => regs[a] * regs[b],
        Op::Muli => regs[a] * b,
        Op::Banr => regs[a] & regs[b],
        Op::Bani => regs[a] & b,
        Op::Borr => regs[a] | regs[b],
        Op::Bori => regs[a] | b,
        Op::Setr => regs[a],
        Op::Seti => a,
        Op::Gtir => (a > regs[b]) as usize,
        Op::Gtri => (regs[a] > b) as usize,
        Op::Gtrr => (regs[a] > regs[b]) as usize,
        Op::Eqir => (a == regs[b]) as usize,
        Op::Eqri => (regs[a] == b) as usize,
        Op::Eqrr => (regs[a] == regs[b]) as usize,
    };
    regs[c] = res;
    regs
}
