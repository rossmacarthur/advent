use advent::prelude::*;

fn parse_sorted_nums(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .sorted_unstable()
        .collect()
}

fn parse_input(input: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
    input
        .lines()
        .map(|card| {
            let (_, nums) = card.split_once(": ").unwrap();
            let (winning, mine) = nums.split_once(" | ").unwrap();
            let winning = parse_sorted_nums(winning);
            let mine = parse_sorted_nums(mine);
            (winning, mine)
        })
        .collect()
}

fn default_input() -> Vec<(Vec<i64>, Vec<i64>)> {
    parse_input(include_input!(2023 / 04))
}

fn intersection_count<T: Ord>(left: &[T], right: &[T]) -> usize {
    let (mut count, mut i, mut j) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        match left[i].cmp(&right[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
        }
    }
    count
}

fn part1(input: Vec<(Vec<i64>, Vec<i64>)>) -> usize {
    input
        .into_iter()
        .map(|(winning, mine)| {
            let n = intersection_count(&winning, &mine);
            2_usize.pow((n - 1) as u32)
        })
        .sum()
}

fn part2(input: Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    let mut cards = vec![1; input.len()];
    input
        .into_iter()
        .enumerate()
        .map(|(i, (winning, mine))| {
            let n = intersection_count(&winning, &mine);
            let m = cards[i];
            for j in 0..n {
                cards[i + j + 1] += m;
            }
            m
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );
    assert_eq!(part1(input.clone()), 13);
    assert_eq!(part2(input), 30);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 25651);
    assert_eq!(part2(input), 19499881);
}
