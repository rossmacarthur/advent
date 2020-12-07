const INPUT: &str = include_str!("input/day01.txt");

pub fn default_input() -> Vec<u64> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[u64]) -> u64 {
    input.iter().map(|mass| (mass / 3) - 2).sum()
}

fn calc_fuel(mass: &u64) -> u64 {
    if *mass == 0 {
        0
    } else {
        let fuel = (mass / 3).saturating_sub(2);
        fuel + calc_fuel(&fuel)
    }
}

pub fn part2(input: &[u64]) -> u64 {
    input.iter().map(calc_fuel).sum()
}
