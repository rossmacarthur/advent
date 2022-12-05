use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<VecDeque<u8>>, Vec<Move>) {
    let (s, p) = input.split_once("\n\n").unwrap();

    let stacks: Vec<VecDeque<_>> = s
        .lines()
        .next_back()
        .unwrap()
        .match_indices(|c: char| c.is_ascii_digit())
        .map(|(m, _)| {
            s.lines()
                .rev()
                .skip(1)
                .filter_map(|line| {
                    line.as_bytes().get(m).and_then(|&b| match b {
                        b'A'..=b'Z' => Some(b),
                        b' ' => None,
                        b => panic!("unexpected byte `{}`", b),
                    })
                })
                .collect()
        })
        .collect();

    let moves = regex!(r"move (\d+) from (\d+) to (\d+)")
        .captures_iter(p)
        .map(|caps| {
            let quantity = caps[1].parse().unwrap();
            let from = caps[2].parse::<usize>().unwrap() - 1;
            let to = caps[3].parse::<usize>().unwrap() - 1;
            Move { quantity, from, to }
        })
        .collect();

    (stacks, moves)
}

fn default_input() -> (Vec<VecDeque<u8>>, Vec<Move>) {
    parse_input(include_str!("input/05.txt"))
}

#[derive(Debug, Clone)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn top(stacks: &[VecDeque<u8>]) -> String {
    let bs: Vec<_> = stacks.iter().filter_map(|s| s.back().copied()).collect();
    String::from_utf8(bs).unwrap()
}

fn part1((mut stacks, moves): (Vec<VecDeque<u8>>, Vec<Move>)) -> String {
    for m in moves {
        for _ in 0..m.quantity {
            let x = stacks[m.from].pop_back().unwrap();
            stacks[m.to].push_back(x);
        }
    }
    top(&stacks)
}

fn part2((mut stacks, moves): (Vec<VecDeque<u8>>, Vec<Move>)) -> String {
    for m in moves {
        for _ in 0..m.quantity {
            let x = stacks[m.from].pop_back().unwrap();
            stacks[m.to].push_front(x);
        }
        stacks[m.to].rotate_left(m.quantity);
    }
    top(&stacks)
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );
    assert_eq!(part1(input.clone()), "CMZ");
    assert_eq!(part2(input), "MCD");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "FRDSQRRCD");
    assert_eq!(part2(input), "HRFTQVWNN");
}
