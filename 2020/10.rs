use itertools::Itertools;

fn parse_input(input: &str) -> Vec<u64> {
    let mut j: Vec<_> = input.split_whitespace().map(str::parse).map(Result::unwrap).collect();
    j.insert(0, 0);
    j.sort_unstable();
    j.push(j.last().unwrap() + 3);
    j
}

fn default_input() -> Vec<u64> {
    parse_input(include_str!("input/10.txt"))
}

fn part1(joltages: &[u64]) -> u64 {
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

fn part2(joltages: &[u64]) -> u64 {
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

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input("16 10 15 5 1 11 7 19 6 12 4");
    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 8);
}

#[test]
fn example2() {
    let input = parse_input(
        "28 33 18 42 31 14 46 20 48 47 24 23 49 45
         19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3"
    );
    assert_eq!(part1(&input), 220);
    assert_eq!(part2(&input), 19208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 1984);
    assert_eq!(part2(&input), 3543369523456);
}
