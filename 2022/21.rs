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
                    let op = it.next().unwrap();
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
    Yell(f64),
    Op(&'a str, &'a str, &'a str),
}

fn eval(monkeys: &HashMap<&str, Action<'_>>, name: &str) -> f64 {
    match monkeys[name] {
        Action::Yell(n) => n,
        Action::Op(lhs, op, rhs) => {
            let l = eval(monkeys, lhs);
            let r = eval(monkeys, rhs);
            match op {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => l / r,
                _ => unreachable!(),
            }
        }
    }
}

fn eval_with(monkeys: &mut HashMap<&str, Action<'_>>, humn: f64) -> f64 {
    monkeys.insert("humn", Action::Yell(humn));
    eval(monkeys, "root")
}

fn part1(monkeys: HashMap<&str, Action<'_>>) -> i64 {
    eval(&monkeys, "root") as i64
}

fn part2(mut monkeys: HashMap<&str, Action<'_>>) -> i64 {
    // Flip the operator for root so that we can solve for root = 0
    match monkeys.get_mut("root") {
        Some(Action::Op(_, op @ "+", _)) => *op = "-",
        _ => panic!("unexpected value for `root`"),
    }

    // Since there is only one instance of humn and root there should be a
    // linear relationship. Let's call humn `x` and root `y`.

    // Firstly, we must figure out the slope of the linear equation in case the
    // relationship between x and y is inversely proportional
    let mut x0 = 0.0;
    let mut x1 = 1e25;
    let y0 = eval_with(&mut monkeys, x0);
    let y1 = eval_with(&mut monkeys, x1);
    let slope = y0.total_cmp(&y1);

    // Binary search to solve for the x intercept
    while x0 < x1 {
        let x = (x0 + x1) / 2.;
        match eval_with(&mut monkeys, x).total_cmp(&0.0) {
            Ordering::Equal => return x as i64,
            o => {
                if o == slope {
                    // The ordering is the same as the slope which means we're
                    // on the right track so increase the lower bound
                    x0 = x
                } else {
                    // Otherwise we've overshot so decrease the upper bound
                    x1 = x
                }
            }
        }
    }
    panic!("no solution found")
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
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
