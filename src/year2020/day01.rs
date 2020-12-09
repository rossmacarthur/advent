use std::cmp::Ordering::*;

use rand::Rng;

const INPUT: &str = include_str!("input/day01.txt");

const SUM: u64 = 2020;

pub fn default_input() -> Vec<u64> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn random_input(count: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut input = vec![0; count];
    rng.fill(input.as_mut_slice());
    input
}

pub fn part1(input: &[u64]) -> Option<u64> {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        let a = numbers[left];
        let b = numbers[right];
        let sum = a + b;
        match sum.cmp(&SUM) {
            Less => left += 1,
            Greater => right -= 1,
            Equal => return Some(a * b),
        }
    }
    None
}

pub fn part2(input: &[u64]) -> Option<u64> {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();
    for i in 0..numbers.len() - 2 {
        let mut left = i + 1;
        let mut right = numbers.len() - 1;
        while left < right {
            let a = numbers[left];
            let b = numbers[right];
            let c = numbers[i];
            let sum = a + b + c;
            match sum.cmp(&SUM) {
                Less => left += 1,
                Greater => right -= 1,
                Equal => return Some(a * b * c),
            }
        }
    }
    None
}
