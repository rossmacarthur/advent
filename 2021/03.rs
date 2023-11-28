use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s| i64::from_str_radix(s, 2))
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2021 / 03))
}

fn partition(values: &[i64], bit: i64) -> (Vec<i64>, Vec<i64>) {
    values.iter().partition(|&v| (v & 1 << bit) > 0)
}

fn criteria<F>(values: &[i64], bits: i64, cmp: F) -> i64
where
    F: Fn(usize, usize) -> bool,
{
    let mut values = values.to_vec();
    for bit in (0..bits).rev() {
        let (ones, zeros) = partition(&values, bit);
        if cmp(ones.len(), zeros.len()) {
            values = ones;
        } else {
            values = zeros;
        }
        if let [value] = *values {
            return value;
        }
    }
    unreachable!()
}

fn part1(values: Vec<i64>, bits: i64) -> i64 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..bits {
        let (ones, zeros) = partition(&values, bit);
        if ones.len() > zeros.len() {
            epsilon |= 1 << bit;
        } else {
            gamma |= 1 << bit;
        }
    }
    gamma * epsilon
}

fn part2(values: Vec<i64>, bits: i64) -> i64 {
    let o2 = criteria(&values, bits, |ones, zeros| ones >= zeros);
    let co2 = criteria(&values, bits, |ones, zeros| ones < zeros);
    o2 * co2
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i, 12))
        .part(|i| part2(i, 12))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    );
    assert_eq!(part1(input.clone(), 5), 198);
    assert_eq!(part2(input, 5), 230);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 12), 3958484);
    assert_eq!(part2(input, 12), 1613181);
}
