use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/09.txt"))
}

fn part1(input: Vec<i64>, size: usize) -> i64 {
    input
        .windows(size + 1)
        .find_map(|window| {
            for i in 0..(size - 1) {
                for j in (i + 1)..size {
                    if window[i] + window[j] == window[size] {
                        return None;
                    }
                }
            }
            Some(window[size])
        })
        .unwrap()
}

fn part2(input: Vec<i64>, invalid: i64) -> i64 {
    let mut i = 0;
    let mut j = 1;
    while j < input.len() {
        let sum: i64 = input[i..j].iter().sum();
        match sum.cmp(&invalid) {
            Ordering::Less => j += 1,
            Ordering::Greater => i += 1,
            Ordering::Equal => {
                let [min, max] = input[i..j]
                    .iter()
                    .fold([i64::MAX, i64::MIN], |[a, z], &i| [min(a, i), max(z, i)]);
                return min + max;
            }
        }
    }
    unreachable!()
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i, 25))
        .part(|i| part2(i, 70639851))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input =
        parse_input("35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576");
    assert_eq!(part1(input.clone(), 5), 127);
    assert_eq!(part2(input, 127), 62);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 25), 70639851);
    assert_eq!(part2(input, 70639851), 8249240);
}
