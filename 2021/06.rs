fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_str!("input/06.txt"))
}

fn solve(fish: &[usize], days: usize) -> usize {
    let mut counts = [0; 9];
    for age in fish {
        counts[*age] += 1;
    }
    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum()
}

fn part1(fish: &[usize]) -> usize {
    solve(fish, 80)
}

fn part2(fish: &[usize]) -> usize {
    solve(fish, 256)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("3,4,3,1,2");
    assert_eq!(solve(&input, 18), 26);
    assert_eq!(part2(&input), 26984457539);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 360610);
    assert_eq!(part2(&input), 1631629590423);
}