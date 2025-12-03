use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn default_input() -> Vec<Vec<u8>> {
    parse_input(include_input!(2025 / 03))
}

fn max_joltage<const B: usize>(bank: Vec<u8>) -> i64 {
    let dpos = bank.len() - B + 1;
    let mut output = [0; B];
    let mut pos = 0;
    for (b, slot) in output.iter_mut().enumerate() {
        pos = (pos..(b + dpos))
            .max_by_key(|&i| (bank[i], Reverse(i)))
            .unwrap();
        *slot = bank[pos];
        pos += 1;
    }
    output
        .iter()
        .rev()
        .enumerate()
        .map(|(pow, &j)| 10_i64.pow(pow as u32) * (j as i64))
        .sum()
}

fn part1(banks: Vec<Vec<u8>>) -> i64 {
    banks.into_iter().map(max_joltage::<2>).sum()
}

fn part2(banks: Vec<Vec<u8>>) -> i64 {
    banks.into_iter().map(max_joltage::<12>).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
987654321111111
811111111111119
234234234234278
818181911112111",
    );
    assert_eq!(part1(input.clone()), 357);
    assert_eq!(part2(input), 3121910778619);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 17613);
    assert_eq!(part2(input), 175304218462560);
}
