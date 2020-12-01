use std::cmp::Ordering::*;

use rand::Rng;

const INPUT: &str = include_str!("day01.txt");

const SUM: u32 = 2020;

pub fn default_input() -> Vec<u32> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn random_input(count: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut input = vec![0; count];
    rng.fill(input.as_mut_slice());
    input
}

pub fn solve_sum_two(mut numbers: Vec<u32>) -> Option<(u32, u32)> {
    numbers.sort_unstable();
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        let a = numbers[left];
        let b = numbers[right];
        let sum = a + b;
        match sum.cmp(&SUM) {
            Less => {
                left += 1;
            }
            Greater => {
                right -= 1;
            }
            Equal => return Some((a, b)),
        }
    }
    None
}

pub fn solve_sum_three(mut numbers: Vec<u32>) -> Option<(u32, u32, u32)> {
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
                Less => {
                    left += 1;
                }
                Greater => {
                    right -= 1;
                }
                Equal => return Some((a, b, c)),
            }
        }
    }
    None
}
