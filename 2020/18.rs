const INPUT: &str = include_str!("input/18.txt");

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    LeftPar,
    RightPar,
    Op(Operator),
    Num(u64),
}

use Operator::*;
use Token::*;

pub fn default_input() -> Vec<Vec<Token>> {
    INPUT
        .lines()
        .map(|line| {
            line.replace("(", " ( ")
                .replace(")", " ) ")
                .split_whitespace()
                .map(|s| match s {
                    "(" => LeftPar,
                    ")" => RightPar,
                    "+" => Op(Add),
                    "*" => Op(Mul),
                    s => Num(s.parse().unwrap()),
                })
                .collect()
        })
        .collect()
}

fn find_right_par(expr: &[Token]) -> usize {
    let mut count = 0;
    for (i, token) in expr.iter().enumerate() {
        match token {
            LeftPar => count += 1,
            RightPar => count -= 1,
            _ => {}
        }
        if count == 0 {
            return i;
        }
    }
    unreachable!()
}

fn parenthesize(expr: &[Token]) -> Vec<Token> {
    let left = [LeftPar, LeftPar, LeftPar];
    let right = [RightPar, RightPar, RightPar];
    let add = [RightPar, Op(Add), LeftPar];
    let mul = [RightPar, RightPar, Op(Mul), LeftPar, LeftPar];
    left.iter()
        .chain(expr.iter().enumerate().flat_map(|(i, token)| match token {
            LeftPar => left.iter(),
            RightPar => right.iter(),
            Op(Add) => add.iter(),
            Op(Mul) => mul.iter(),
            _ => expr[i..i + 1].iter(),
        }))
        .chain(right.iter())
        .copied()
        .collect()
}

fn evaluate(expr: &[Token]) -> u64 {
    let mut left = 0;
    let mut result = 0;
    let mut op = Add;
    while left < expr.len() {
        match expr[left] {
            LeftPar => {
                let right = left + find_right_par(&expr[left..]);
                let value = evaluate(&expr[left + 1..right]);
                match op {
                    Add => result += value,
                    Mul => result *= value,
                }
                left = right + 1;
                continue;
            }
            RightPar => {
                unreachable!()
            }
            Op(operator) => op = operator,
            Num(value) => match op {
                Add => result += value,
                Mul => result *= value,
            },
        }
        left += 1;
    }
    result
}

pub fn part1(input: &[Vec<Token>]) -> u64 {
    input.iter().map(|expr| evaluate(expr.as_slice())).sum()
}

pub fn part2(input: &[Vec<Token>]) -> u64 {
    input
        .iter()
        .map(|expr| evaluate(&parenthesize(expr.as_slice())))
        .sum()
}
