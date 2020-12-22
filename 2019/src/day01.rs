const INPUT: &str = include_str!("input/day01.txt");

pub fn default_input() -> Vec<u64> {
    INPUT.lines().map(str::parse).map(Result::unwrap).collect()
}

fn fuel_for_mass(mass: u64) -> u64 {
    if mass == 0 {
        0
    } else {
        let fuel = (mass / 3).saturating_sub(2);
        fuel + fuel_for_mass(fuel)
    }
}

pub fn part1(masses: &[u64]) -> u64 {
    masses.iter().map(|mass| (mass / 3) - 2).sum()
}

pub fn part2(masses: &[u64]) -> u64 {
    masses.iter().copied().map(fuel_for_mass).sum()
}

#[test]
fn ex1() {
    assert_eq!(fuel_for_mass(100756), 50346);
}

#[test]
fn default() {
    let masses = default_input();
    assert_eq!(part1(&masses), 3432671);
    assert_eq!(part2(&masses), 5146132);
}
