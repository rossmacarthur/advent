#![allow(clippy::option_map_unit_fn)]

use std::fmt;
use std::str::Chars;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Node> {
    input.lines().map(Node::parse).collect()
}

fn default_input() -> Vec<Node> {
    parse_input(include_str!("input/18.txt"))
}

#[derive(Clone)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Num(u32),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(a, b) => write!(f, "[{:?},{:?}]", a, b),
            Self::Num(v) => write!(f, "{}", v),
        }
    }
}

impl Node {
    fn parse(s: &str) -> Self {
        parse(&mut s.chars(), 0)
    }

    fn pair_mut(&mut self) -> Option<(&mut Node, &mut Node)> {
        match self {
            Node::Pair(left, right) => Some((left, right)),
            Node::Num(_) => None,
        }
    }
}

fn parse(it: &mut Chars<'_>, depth: usize) -> Node {
    if depth == 0 {
        match it.next() {
            Some('[') => {}
            c => panic!("expected open bracket, got `{:?}`", c),
        }
    }
    let mut left = None;
    let mut right = None;
    loop {
        let node = match it.next() {
            Some('[') => parse(it, depth + 1),
            Some(c @ '0'..='9') => Node::Num(c.to_digit(10).unwrap()),
            c => panic!("expected number or open bracket, got `{:?}`", c),
        };
        assert!(left.replace(node).is_none());
        mem::swap(&mut left, &mut right);
        match it.next() {
            Some(',') => continue,
            Some(']') => {
                let left = Box::new(left.unwrap());
                let right = Box::new(right.unwrap());
                break Node::Pair(left, right);
            }
            c => panic!("expected comma or close bracket, got `{:?}`", c),
        }
    }
}

fn explode(node: &mut Node, depth: usize) -> Result<(), (Option<u32>, Option<u32>)> {
    if let Some((left, right)) = node.pair_mut() {
        match (left, right) {
            // A pair of regular numbers at depth 4.
            (Node::Num(m), Node::Num(n)) if depth == 0 => {
                // 💥 Exploding! Simply return the regular values and create a
                // new zero regular node to replace this pair.
                let mn = (Some(*m), Some(*n));
                *node = Node::Num(0);
                return Err(mn);
            }
            // Some other kind of pair.
            (left, right) => {
                // Try explode the left node.
                explode(left, depth - 1).map_err(|(m, n)| {
                    n.map(|v| add_left(right, v));
                    (m, None)
                })?;
                // Try explode the right node.
                explode(right, depth - 1).map_err(|(m, n)| {
                    m.map(|v| add_right(left, v));
                    (None, n)
                })?;
            }
        }
    }
    Ok(())
}

fn add_left(node: &mut Node, v: u32) {
    match node {
        Node::Pair(left, _) => add_left(left, v),
        Node::Num(n) => *n += v,
    }
}

fn add_right(node: &mut Node, v: u32) {
    match node {
        Node::Pair(_, right) => add_right(right, v),
        Node::Num(n) => *n += v,
    }
}

fn split(node: &mut Node) -> Result<(), ()> {
    match node {
        Node::Pair(left, right) => {
            split(left)?;
            split(right)?;
        }
        Node::Num(n) => {
            if *n >= 10 {
                let left = Box::new(Node::Num(*n / 2));
                let right = Box::new(Node::Num((*n + 1) / 2));
                *node = Node::Pair(left, right);
                return Err(());
            }
        }
    }
    Ok(())
}

fn add(a: Node, b: Node) -> Node {
    let mut node = Node::Pair(Box::new(a), Box::new(b));
    loop {
        if explode(&mut node, 4).is_err() {
            continue;
        };
        if split(&mut node).is_err() {
            continue;
        }
        break node;
    }
}

fn magnitude(node: Node) -> u32 {
    match node {
        Node::Pair(left, right) => 3 * magnitude(*left) + 2 * magnitude(*right),
        Node::Num(n) => n,
    }
}

fn part1(nodes: Vec<Node>) -> u32 {
    nodes.into_iter().reduce(add).map(magnitude).unwrap()
}

fn part2(nodes: Vec<Node>) -> u32 {
    nodes.into_iter().permutations(2).map(part1).max().unwrap()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
    );
    assert_eq!(
        format!("{:?}", input.into_iter().reduce(add).unwrap()),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    );
}

#[test]
fn example2() {
    let input = parse_input(
        "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    );
    assert_eq!(part1(input.clone()), 4140);
    assert_eq!(part2(input), 3993);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3551);
    assert_eq!(part2(input), 4555);
}
