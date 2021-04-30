use std::cmp::Ordering::*;

fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(str::parse).map(Result::unwrap).collect()
}

fn default_input() -> Vec<u64> {
    parse_input(include_str!("input/09.txt"))
}

fn part1(input: &[u64], size: usize) -> u64 {
    input.windows(size + 1)
        .find(|window| {
            for i in 0..(size - 1) {
                for j in (i + 1)..size {
                    if window[i] + window[j] == window[size] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|window| window[size])
        .unwrap()
}

fn part2(input: &[u64], invalid: u64) -> u64 {
    let mut i = 0;
    let mut j = 1;
    while j < input.len() {
        let sum: u64 = input[i..j].iter().sum();
        match sum.cmp(&invalid) {
            Less => j += 1,
            Greater => i += 1,
            Equal => {
                return input[i..j].iter().max().unwrap()
                    + input[i..j].iter().min().unwrap();
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input, 25));
    run.part(|| part2(&input, 70639851));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576");
    assert_eq!(part1(&input, 5), 127);
    assert_eq!(part2(&input, 127), 62);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input, 25), 70639851);
    assert_eq!(part2(&input, 70639851), 8249240);
}
