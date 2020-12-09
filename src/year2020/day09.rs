use std::cmp::Ordering::*;

const INPUT: &str = include_str!("input/day09.txt");
const SUM: usize = 70639851;

pub fn default_input() -> Vec<usize> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

fn has_sum_nums(preamble: &[usize], value: usize) -> bool {
    for (i, left) in preamble.iter().enumerate() {
        for right in &preamble[i + 1..] {
            if left + right == value {
                return true;
            }
        }
    }
    return false;
}

pub fn part1(input: &[usize]) -> Option<usize> {
    for slice in input.windows(26) {
        let value = slice[25];
        if !has_sum_nums(&slice[..25], value) {
            return Some(value);
        }
    }
    None
}

pub fn part2(input: &[usize]) -> Option<usize> {
    let mut left = 0;
    let mut right = 1;
    while right < input.len() {
        match input[left..right].iter().sum::<usize>().cmp(&SUM) {
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
