const INPUT: &'static str = include_str!("day01.txt");

pub fn input() -> Vec<u32> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_sum_two(numbers: &[u32]) -> Option<(u32, u32)> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            if a + b == 2020 {
                return Some((a, b));
            }
        }
    }
    None
}

pub fn solve_sum_three(numbers: &[u32]) -> Option<(u32, u32, u32)> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                let a = numbers[i];
                let b = numbers[j];
                let c = numbers[k];
                if a + b + c == 2020 {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}
