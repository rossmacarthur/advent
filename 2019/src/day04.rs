use std::collections::VecDeque;
use std::ops::Range;

use itertools::Itertools;

pub fn default_input() -> Range<u64> {
    123257..647015
}

fn digits(mut num: u64) -> Vec<i8> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as i8);
        num /= 10;
    }
    digits
}

fn has_increasing_digits(digits: &[i8]) -> bool {
    digits.iter().tuple_windows().all(|(x, y)| y >= x)
}

pub fn part1(input: &Range<u64>) -> usize {
    input
        .clone()
        .filter(|&num| {
            let digits = digits(num);
            has_increasing_digits(&digits) && digits.iter().tuple_windows().any(|(x, y)| x == y)
        })
        .count()
}

pub fn part2(input: &Range<u64>) -> usize {
    input
        .clone()
        .filter(|&num| {
            let digits = digits(num);
            let mut digits_pad: VecDeque<_> = digits.clone().into();
            digits_pad.push_back(-1);
            digits_pad.push_front(-1);
            has_increasing_digits(&digits)
                && digits_pad
                    .iter()
                    .tuple_windows()
                    .any(|(w, x, y, z)| x == y && w != x && y != z)
        })
        .count()
}
