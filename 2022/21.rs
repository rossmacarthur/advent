use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<&str, Action<'_>> {
    input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            let mut it = rest.split_whitespace();
            let action = match it.next().unwrap() {
                n if n.chars().all(|c| c.is_ascii_digit()) => Action::Yell(n.parse().unwrap()),
                lhs => {
                    let op = match it.next().unwrap() {
                        "+" => Op::Add,
                        "-" => Op::Sub,
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        o => panic!("unknown operator `{o}`"),
                    };
                    let rhs = it.next().unwrap();
                    Action::Op(lhs, op, rhs)
                }
            };
            (name, action)
        })
        .collect()
}

fn default_input() -> HashMap<&'static str, Action<'static>> {
    parse_input(include_str!("input/21.txt"))
}

#[derive(Debug, Clone, Copy)]
enum Action<'a> {
    Yell(i64),
    Op(&'a str, Op, &'a str),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn eval(monkeys: &HashMap<&str, Action<'_>>, name: &str) -> i64 {
    match monkeys[name] {
        Action::Yell(n) => n,
        Action::Op(lhs, op, rhs) => {
            let l = eval(monkeys, lhs);
            let r = eval(monkeys, rhs);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    }
}

fn to_satisfy(monkeys: &HashMap<&str, Action<'_>>, name: &str) -> bool {
    if name == "humn" {
        return true;
    }
    match monkeys[name] {
        Action::Yell(_) => false,
        Action::Op(lhs, _, rhs) => to_satisfy(monkeys, lhs) || to_satisfy(monkeys, rhs),
    }
}

fn satisfy(monkeys: &HashMap<&str, Action<'_>>, name: &str, v: i64) -> i64 {
    match monkeys[name] {
        Action::Yell(_) => v,
        Action::Op(lhs, op, rhs) => {
            // Check which side of the equation has the `humn` so we know which
            // side to evaluate and which side to try satisfy (recurse).
            let hl = to_satisfy(monkeys, lhs);
            let hr = to_satisfy(monkeys, rhs);
            assert!(hl != hr, "unsolvable");

            // Evaluate the side without the `humn` and calculate a new value
            // that we can use to satisfy the other side. The only tricky part
            // here is if the operator is non-commutative i.e. subtraction or
            // division then the reverse operation is not simply using the other
            // operator. For example
            //
            //    eval(lhs) / rhs = value
            // => rhs = eval(lhs) / value
            //
            match op {
                Op::Add if hl => satisfy(monkeys, lhs, v - eval(monkeys, rhs)),
                Op::Sub if hl => satisfy(monkeys, lhs, v + eval(monkeys, rhs)),
                Op::Mul if hl => satisfy(monkeys, lhs, v / eval(monkeys, rhs)),
                Op::Div if hl => satisfy(monkeys, lhs, v * eval(monkeys, rhs)),
                Op::Add => satisfy(monkeys, rhs, v - eval(monkeys, lhs)),
                Op::Sub => satisfy(monkeys, rhs, eval(monkeys, lhs) - v),
                Op::Mul => satisfy(monkeys, rhs, v / eval(monkeys, lhs)),
                Op::Div => satisfy(monkeys, rhs, eval(monkeys, lhs) / v),
            }
        }
    }
}

fn part1(monkeys: HashMap<&str, Action<'_>>) -> i64 {
    eval(&monkeys, "root")
}

fn part2(mut monkeys: HashMap<&str, Action<'_>>) -> i64 {
    // Flip the add operator for root so that we can solve for root = 0
    match monkeys.get_mut("root") {
        Some(Action::Op(_, op @ Op::Add, _)) => *op = Op::Sub,
        _ => panic!("unexpected value for `root`"),
    }
    satisfy(&monkeys, "root", 0)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
    );
    assert_eq!(part1(input.clone()), 152);
    assert_eq!(part2(input), 301);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 24947355373338);
    assert_eq!(part2(input), 3876907167495);
}
