use std::cmp::Ordering::*;

const INPUT: &str = include_str!("input/day09.txt");
const SUM: u64 = 70639851;

pub fn default_input() -> Vec<u64> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

fn has_sum_nums(preamble: &[u64], value: u64) -> bool {
    for (i, left) in preamble.iter().enumerate() {
        for right in &preamble[i + 1..] {
            if left + right == value {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &[u64]) -> Option<u64> {
    for slice in input.windows(26) {
        let value = slice[25];
        if !has_sum_nums(&slice[..25], value) {
            return Some(value);
        }
    }
    None
}

pub fn part2(input: &[u64]) -> Option<u64> {
    let mut left = 0;
    let mut right = 1;
    while right < input.len() {
        match input[left..right].iter().sum::<u64>().cmp(&SUM) {
            Less => right += 1,
            Greater => left += 1,
            Equal => {
                return Some(
                    input[left..right].iter().max().unwrap()
                        + input[left..right].iter().min().unwrap(),
                );
            }
        }
    }
    None
}
