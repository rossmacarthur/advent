use itertools::Itertools;

const INPUT: &str = include_str!("input/day10.txt");

pub fn default_input() -> Vec<u64> {
    let mut joltages: Vec<_> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    joltages.insert(0, 0);
    joltages.sort_unstable();
    joltages.push(joltages.last().unwrap() + 3);
    joltages
}

pub fn part1(joltages: &[u64]) -> u64 {
    let mut ones = 0;
    let mut threes = 0;
    for (curr, next) in joltages.iter().tuple_windows() {
        match next - curr {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }
    ones * threes
}

pub fn part2(joltages: &[u64]) -> u64 {
    let mut dp = vec![1];
    for (i, joltage) in joltages.iter().enumerate().skip(1) {
        dp.push(
            (i.saturating_sub(3)..i)
                .filter(|j| joltage - joltages[*j] <= 3)
                .map(|j| dp[j])
                .sum(),
        )
    }
    *dp.last().unwrap()
}
