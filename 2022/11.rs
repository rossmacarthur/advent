use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Monkey> {
    let re = regex!(
        r"Monkey \d+:
\s*Starting items: (?P<items>[\s\d,]+)
\s*Operation: new = old (?P<op>(\*|\+)) (?P<n>old|\d+)
\s*Test: divisible by (?P<div>\d+)
\s*If true: throw to monkey (?P<true>\d+)
\s*If false: throw to monkey (?P<false>\d+)"
    );
    input
        .split("\n\n")
        .map(|m| {
            let caps = re.captures(m).unwrap();
            let items = caps["items"]
                .split(", ")
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            let op = match (&caps["op"], &caps["n"]) {
                ("*", "old") => Op::Square,
                ("+", n) => Op::Add(n.parse().unwrap()),
                ("*", n) => Op::Mul(n.parse().unwrap()),
                _ => unreachable!(),
            };
            let div = caps["div"].parse().unwrap();
            let mt = caps["true"].parse().unwrap();
            let mf = caps["false"].parse().unwrap();
            Monkey {
                items,
                op,
                div,
                mt,
                mf,
            }
        })
        .collect()
}

fn default_input() -> Vec<Monkey> {
    parse_input(include_str!("input/11.txt"))
}

#[derive(Debug, Clone)]
struct Monkey {
    /// The items held by this monkey
    items: VecDeque<usize>,
    /// The operation that is applied when inspecting an item
    op: Op,
    /// The divisor to test the worry level with
    div: usize,
    /// The monkey to throw the item to if the test passes
    mt: usize,
    /// The monkey to throw the item to if the test fails
    mf: usize,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

fn solve<F>(mut monkeys: Vec<Monkey>, rounds: usize, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(old) = monkeys[m].items.pop_front() {
                let Monkey {
                    op, div, mt, mf, ..
                } = monkeys[m];
                let new = f(match op {
                    Op::Add(n) => old + n,
                    Op::Mul(n) => old * n,
                    Op::Square => old * old,
                });
                let to = if new % div == 0 { mt } else { mf };
                monkeys[to].items.push_back(new);
                inspects[m] += 1;
            }
        }
    }

    inspects.into_iter().sorted().rev().take(2).product()
}

fn part1(monkeys: Vec<Monkey>) -> usize {
    solve(monkeys, 20, |w| w / 3)
}

fn part2(monkeys: Vec<Monkey>) -> usize {
    // We need to calculate `worry mod m` so that the worry amounts don't get
    // out of hand. Using the multiplication of all the divisors as `m` will
    // ensure that the tests still remain the same and all the items will get
    // passed to the same monkeys 🐒🐒🐒
    let m: usize = monkeys.iter().map(|m| m.div).product();
    solve(monkeys, 10_000, |w| w % m)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
    );
    assert_eq!(part1(input.clone()), 10605);
    assert_eq!(part2(input), 2713310158);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 66802);
    assert_eq!(part2(input), 21800916620);
}
