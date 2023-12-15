use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<Node, [Node; 2]>) {
    let (instrs, nodes) = input.split_once("\n\n").unwrap();

    let instrs = instrs
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("unexpected direction `{}`", c),
        })
        .collect();

    let nodes = nodes
        .lines()
        .map(|line| {
            let re = regex!(r"(?P<from>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)");
            let caps = re.captures(line).unwrap();
            let parse_node = |s: &str| s.bytes().collect_array();
            let from = parse_node(&caps["from"]);
            let left = parse_node(&caps["left"]);
            let right = parse_node(&caps["right"]);
            (from, [left, right])
        })
        .collect();

    (instrs, nodes)
}

fn default_input() -> (Vec<Direction>, HashMap<Node, [Node; 2]>) {
    parse_input(include_input!(2023 / 08))
}

type Node = [u8; 3];

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn steps<F>(instrs: &[Direction], nodes: &HashMap<Node, [Node; 2]>, start: Node, end_fn: F) -> i64
where
    F: Fn(&Node) -> bool,
{
    let count = instrs
        .iter()
        .cycle()
        .scan(start, |n, d| {
            let [l, r] = nodes[n];
            match d {
                Direction::Left => *n = l,
                Direction::Right => *n = r,
            }
            Some(*n)
        })
        .take_while(|n| !end_fn(n))
        .count();
    count as i64 + 1
}

fn part1((instrs, nodes): (Vec<Direction>, HashMap<Node, [Node; 2]>)) -> i64 {
    steps(&instrs, &nodes, *b"AAA", |n| n == b"ZZZ")
}

fn part2((instrs, nodes): (Vec<Direction>, HashMap<Node, [Node; 2]>)) -> i64 {
    nodes
        .keys()
        .copied()
        .filter(|k| k.ends_with(b"A"))
        .map(|n| steps(&instrs, &nodes, n, |n| n.ends_with(b"Z")))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn example1() {
    let input = parse_input(
        "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    );
    assert_eq!(part1(input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    );
    assert_eq!(part1(input), 6);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    );
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 21389);
    assert_eq!(part2(input), 21083806112641);
}
