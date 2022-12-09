fn default_input() -> Vec<u64> {
    include_str!("input/01.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn fuel_for_mass(mass: u64) -> u64 {
    if mass == 0 {
        0
    } else {
        let fuel = (mass / 3).saturating_sub(2);
        fuel + fuel_for_mass(fuel)
    }
}

fn part1(masses: Vec<u64>) -> u64 {
    masses.into_iter().map(|mass| (mass / 3) - 2).sum()
}

fn part2(masses: Vec<u64>) -> u64 {
    masses.into_iter().map(fuel_for_mass).sum()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    assert_eq!(fuel_for_mass(100756), 50346);
}

#[test]
fn default() {
    let masses = default_input();
    assert_eq!(part1(masses.clone()), 3432671);
    assert_eq!(part2(masses), 5146132);
}
