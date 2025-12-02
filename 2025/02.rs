use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[u64; 2]> {
    input
        .trim()
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap()
        })
        .collect()
}

fn default_input() -> Vec<[u64; 2]> {
    parse_input(include_input!(2025 / 02))
}

/// Generates all invalid IDs in the given range with the given number of repeats.
fn invalid_ids([lo, hi]: [u64; 2], reps: u32) -> impl Iterator<Item = u64> + use<> {
    let lo_digits = lo.ilog10() + 1;
    let start = 10u64.pow(max(lo_digits / reps, 1) - 1);
    (start..)
        .map(move |seq| -> u64 {
            let seq_digits = seq.ilog10() + 1;
            (0..reps).map(|p| seq * 10u64.pow(seq_digits * p)).sum()
        })
        .skip_while(move |&id| id < lo)
        .take_while(move |&id| id <= hi)
}

fn part1(ranges: Vec<[u64; 2]>) -> u64 {
    ranges
        .into_iter()
        .flat_map(|range| invalid_ids(range, 2))
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}

fn part2(ranges: Vec<[u64; 2]>) -> u64 {
    ranges
        .into_iter()
        .flat_map(|[lo, hi]| {
            let hi_digits = hi.ilog10() + 1;
            (2..=hi_digits).flat_map(move |reps| invalid_ids([lo, hi], reps))
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    );
    assert_eq!(part1(input.clone()), 1227775554);
    assert_eq!(part2(input), 4174379265);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 52316131093);
    assert_eq!(part2(input), 69564213293);
}
