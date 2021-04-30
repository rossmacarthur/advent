use Operator::*;
use Token::*;

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input
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

fn default_input() -> Vec<Vec<Token>> {
    parse_input(include_str!("input/18.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    LeftPar,
    RightPar,
    Op(Operator),
    Num(u64),
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

fn part1(input: &[Vec<Token>]) -> u64 {
    input.iter().map(|expr| evaluate(expr.as_slice())).sum()
}

fn part2(input: &[Vec<Token>]) -> u64 {
    input
        .iter()
        .map(|expr| evaluate(&parenthesize(expr.as_slice())))
        .sum()
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
    let test_cases = &[
        ("1 + 2 * 3 + 4 * 5 + 6", 71, 231),
        ("2 * 3 + (4 * 5)", 26, 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632, 23340),
    ];
    for &(input, r1, r2) in test_cases {
        let input = parse_input(input);
        assert_eq!(part1(&input), r1);
        assert_eq!(part2(&input), r2);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 7293529867931);
    assert_eq!(part2(&input), 60807587180737);
}
