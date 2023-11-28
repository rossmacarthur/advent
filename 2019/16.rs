use std::ops::Rem;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| {
            assert!(c.is_ascii_digit());
            c as i64 - '0' as i64
        })
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2019 / 16))
}

const PATTERN: &[i64] = &[0, 1, 0, -1];

fn from_digit(d: i64) -> char {
    assert!((0..10).contains(&d));
    (d as u8 + 0x30) as char
}

fn fft(mut signal: Vec<i64>) -> Vec<i64> {
    for _ in 0..100 {
        signal = (1..=signal.len())
            .map(|i| {
                PATTERN
                    .iter()
                    .flat_map(|p| iter::repeat(p).take(i))
                    .cycle()
                    .skip(1)
                    .zip(&signal)
                    .map(|(p, x)| p * x)
                    .sum::<i64>()
                    .abs()
                    .rem(10)
            })
            .collect();
    }
    signal
}

fn fft2(mut signal: Vec<i64>) -> Vec<i64> {
    signal.reverse();
    for _ in 0..100 {
        let mut acc = 0;
        for x in &mut signal {
            acc += *x;
            *x = acc.abs().rem(10);
        }
    }
    signal.reverse();
    signal
}

fn part1(signal: Vec<i64>) -> String {
    fft(signal).into_iter().take(8).map(from_digit).collect()
}

fn part2(signal: Vec<i64>) -> String {
    let offset = signal.iter().take(7).fold(0, |acc, x| x + acc * 10) as usize;
    let len = signal.len();
    let signal = signal
        .into_iter()
        .cycle()
        .skip(offset)
        .take(10_000 * len - offset)
        .collect();
    fft2(signal).into_iter().take(8).map(from_digit).collect()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input("80871224585914546619083218645595");
    assert_eq!(part1(input), "24176176");

    let input = parse_input("19617804207202209144916044189917");
    assert_eq!(part1(input), "73745418");

    let input = parse_input("69317163492948606335995924319873");
    assert_eq!(part1(input), "52432133");
}

#[test]
fn example2() {
    let input = parse_input("03036732577212944063491565474664");
    assert_eq!(part2(input), "84462026");

    let input = parse_input("02935109699940807407585447034323");
    assert_eq!(part2(input), "78725270");

    let input = parse_input("03081770884921959731165446850517");
    assert_eq!(part2(input), "53553731");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "22122816");
    assert_eq!(part2(input), "41402171");
}
