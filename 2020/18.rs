use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '+' => Token::Op(Op::Add),
                    '*' => Token::Op(Op::Mul),
                    '0'..='9' => Token::Num(c as i64 - '0' as i64),
                    c => panic!("unexpected character `{}`", c),
                })
                .collect()
        })
        .collect()
}

fn default_input() -> Vec<Vec<Token>> {
    parse_input(include_str!("input/18.txt"))
}

#[derive(Debug, Clone)]
enum Token {
    LeftParen,
    RightParen,
    Op(Op),
    Num(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Mul,
    Add,
}

impl Op {
    fn eq(&self, _: &Op) -> Ordering {
        Ordering::Equal
    }
}

// Evaluates the expression taking into account the operator precedance.
fn eval<F>(tokens: Vec<Token>, prec: F) -> i64
where
    F: Fn(&Op, &Op) -> Ordering,
{
    let mut values = Vec::new();
    for token in to_rpn(tokens, prec) {
        let value = match token {
            Token::Op(op) => {
                let a = values.pop().unwrap();
                let b = values.pop().unwrap();
                match op {
                    Op::Add => a + b,
                    Op::Mul => a * b,
                }
            }
            Token::Num(value) => value,
            t => panic!("unexpected token `{:?}`", t),
        };
        values.push(value);
    }
    values.pop().unwrap()
}

// Converts the token stream from an infix notation expression to reverse polish
// notation taking into account the operator precedance.
//
// See https://en.wikipedia.org/wiki/Shunting-yard_algorithm
fn to_rpn<F>(tokens: Vec<Token>, prec: F) -> Vec<Token>
where
    F: Fn(&Op, &Op) -> Ordering,
{
    let mut output: Vec<Token> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Token::LeftParen => {
                ops.push(token);
            }
            Token::RightParen => {
                while matches!(ops.last(), Some(Token::Op(_))) {
                    output.push(ops.pop().unwrap());
                }
                ops.pop().unwrap();
            }
            Token::Op(ref o1) => {
                while matches!(ops.last(), Some(Token::Op(o2)) if prec(o2, o1).is_ge()) {
                    output.push(ops.pop().unwrap());
                }
                ops.push(token);
            }
            Token::Num(_) => {
                output.push(token);
            }
        }
    }
    output.extend(ops.into_iter().rev());
    output
}

fn part1(input: Vec<Vec<Token>>) -> i64 {
    input.into_iter().map(|ts| eval(ts, Op::eq)).sum()
}

fn part2(input: Vec<Vec<Token>>) -> i64 {
    input.into_iter().map(|ts| eval(ts, Op::cmp)).sum()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let tests = &[
        ("1 + 2 * 3 + 4 * 5 + 6", 71, 231),
        ("2 * 3 + (4 * 5)", 26, 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
        (
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            13632,
            23340,
        ),
    ];
    for &(input, r1, r2) in tests {
        let input = parse_input(input);
        assert_eq!(part1(input.clone()), r1);
        assert_eq!(part2(input), r2);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7293529867931);
    assert_eq!(part2(input), 60807587180737);
}
