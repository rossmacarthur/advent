use std::vec::IntoIter;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .flat_map(|c| {
            let d = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(move |i| (d >> i) & 0b1)
        })
        .collect()
}

fn default_input() -> Vec<u8> {
    parse_input(include_str!("input/16.txt"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: i64,
    type_id: u8,
    payload: Payload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Payload {
    Literal(i64),
    Packets(Vec<Packet>),
}

fn parse_literal(bits: &mut IntoIter<u8>) -> Option<i64> {
    let mut bits = bits.map(i64::from);
    let mut value = 0;
    loop {
        let mark = bits.next()?;
        let part = bits.by_ref().take(4).fold(0, |acc, b| acc * 2 + b);
        value = value << 4 | part;
        if mark == 0 {
            break Some(value);
        }
    }
}

fn parse_packets(bits: &mut IntoIter<u8>) -> Option<Vec<Packet>> {
    let packets = match bits.next()? {
        0 => {
            let count = bits.take(15).map(usize::from).fold(0, |acc, b| acc * 2 + b);
            let mut bits = bits.take(count).collect::<Vec<_>>().into_iter();
            iter::from_fn(|| parse_packet(&mut bits)).collect()
        }
        1 => {
            let count = bits.take(11).map(usize::from).fold(0, |acc, b| acc * 2 + b);
            iter::repeat_with(|| parse_packet(bits).unwrap())
                .take(count)
                .collect()
        }
        _ => unreachable!(),
    };
    Some(packets)
}

fn parse_packet(bits: &mut IntoIter<u8>) -> Option<Packet> {
    let version = bits.take(3).map(i64::from).fold(0, |acc, b| acc * 2 + b);
    let type_id = bits.take(3).fold(0, |acc, b| acc * 2 + b);
    let payload = match type_id {
        4 => Payload::Literal(parse_literal(bits)?),
        _ => Payload::Packets(parse_packets(bits)?),
    };
    Some(Packet {
        version,
        type_id,
        payload,
    })
}

fn sum_versions(packet: &Packet) -> i64 {
    let sum = match &packet.payload {
        Payload::Literal(_) => 0,
        Payload::Packets(packets) => packets.iter().map(sum_versions).sum(),
    };
    packet.version + sum
}

fn eval(packet: &Packet) -> i64 {
    match &packet.payload {
        Payload::Literal(value) => *value,
        Payload::Packets(packets) => match packet.type_id {
            0 => packets.iter().map(eval).sum(),
            1 => packets.iter().map(eval).product(),
            2 => packets.iter().map(eval).min().unwrap(),
            3 => packets.iter().map(eval).max().unwrap(),
            5 => (eval(&packets[0]) > eval(&packets[1])) as i64,
            6 => (eval(&packets[0]) < eval(&packets[1])) as i64,
            7 => (eval(&packets[0]) == eval(&packets[1])) as i64,
            _ => unreachable!(),
        },
    }
}

fn part1(bits: Vec<u8>) -> i64 {
    sum_versions(&parse_packet(&mut bits.into_iter()).unwrap())
}

fn part2(bits: Vec<u8>) -> i64 {
    eval(&parse_packet(&mut bits.into_iter()).unwrap())
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[cfg(test)]
fn literal(version: i64, payload: i64) -> Packet {
    Packet {
        version,
        type_id: 4,
        payload: Payload::Literal(payload),
    }
}

#[test]
fn example1() {
    let input = parse_input("D2FE28");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        literal(6, 2021),
    );
}

#[test]
fn example2() {
    let input = parse_input("38006F45291200");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        Packet {
            version: 1,
            type_id: 6,
            payload: Payload::Packets(vec![literal(6, 10), literal(2, 20)]),
        }
    );
}

#[test]
fn example3() {
    let input = parse_input("EE00D40C823060");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        Packet {
            version: 7,
            type_id: 3,
            payload: Payload::Packets(vec![literal(2, 1), literal(4, 2), literal(1, 3)]),
        }
    );
}

#[test]
fn example4() {
    assert_eq!(part1(parse_input("8A004A801A8002F478")), 16);
    assert_eq!(part1(parse_input("620080001611562C8802118E34")), 12);
    assert_eq!(part1(parse_input("C0015000016115A2E0802F182340")), 23);
    assert_eq!(part1(parse_input("A0016C880162017C3686B18A3D4780")), 31);
}

#[test]
fn example5() {
    assert_eq!(part2(parse_input("C200B40A82")), 3);
    assert_eq!(part2(parse_input("04005AC33890")), 54);
    assert_eq!(part2(parse_input("880086C3E88112")), 7);
    assert_eq!(part2(parse_input("CE00C43D881120")), 9);
    assert_eq!(part2(parse_input("D8005AC2A8F0")), 1);
    assert_eq!(part2(parse_input("F600BC2D8F")), 0);
    assert_eq!(part2(parse_input("9C005AC2F8F0")), 0);
    assert_eq!(part2(parse_input("9C0141080250320F1802104A08")), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 934);
    assert_eq!(part2(input), 912901337844);
}
