use std::iter::Peekable;
use std::str::Bytes;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Packet> {
    input.split_whitespace().map(Packet::parse).collect()
}

fn default_input() -> Vec<Packet> {
    parse_input(include_str!("input/13.txt"))
}

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(u8),
}

impl Packet {
    fn parse(s: &str) -> Self {
        parse_impl(&mut s[1..].bytes().peekable())
    }
}

/// Recursively parses the packet data from the iterator until a corresponding
/// end bracket `]` is found.
fn parse_impl(it: &mut Peekable<Bytes<'_>>) -> Packet {
    let mut list = Vec::new();
    while let Some(b) = it.next() {
        match b {
            // Encountered a number, make sure to include any following digits
            // and append the result to our current list.
            b'0'..=b'9' => {
                let mut num = b - b'0';
                while let Some(b'0'..=b'9') = it.peek() {
                    num = 10 * num + (it.next().unwrap() - b'0');
                }
                list.push(Packet::Num(num));
            }
            // Encountered a nested list, recurse to parse it and append the
            // result to our current list.
            b'[' => list.push(parse_impl(it)),
            // Finished parsing the list, so return it
            b']' => return Packet::List(list),
            // Just continue to the next number or list
            b',' => continue,
            b => panic!("unexpected byte `{b}`"),
        }
    }
    unreachable!()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let slice_cmp = <[Packet]>::cmp;
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => slice_cmp(l, r),
            (Packet::List(l), &Packet::Num(r)) => slice_cmp(l, &[Packet::Num(r)]),
            (&Packet::Num(l), Packet::List(r)) => slice_cmp(&[Packet::Num(l)], r),
        }
    }
}

fn part1(packets: Vec<Packet>) -> usize {
    packets
        .into_iter()
        .array_chunked()
        .enumerate()
        .filter_map(|(i, [left, right])| (left < right).some(i + 1))
        .sum()
}

fn part2(mut packets: Vec<Packet>) -> usize {
    packets.sort();
    let i = packets.binary_search(&Packet::parse("[[2]]")).unwrap_err() + 1;
    let j = packets.binary_search(&Packet::parse("[[6]]")).unwrap_err() + 2;
    i * j
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
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
    );
    assert_eq!(part1(input.clone()), 13);
    assert_eq!(part2(input), 140);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5557);
    assert_eq!(part2(input), 22425);
}
