fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/07.txt"))
}

fn solve<F>(crabs: Vec<i64>, cost: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let max = crabs.iter().copied().max().unwrap();
    (0..=max)
        .map(|h| crabs.iter().map(|crab| cost((crab - h).abs())).sum())
        .min()
        .unwrap()
}

fn part1(crabs: Vec<i64>) -> i64 {
    solve(crabs, |d| d)
}

fn part2(crabs: Vec<i64>) -> i64 {
    solve(crabs, |d| d * (1 + d) / 2)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}
#[test]
fn example() {
    let input = parse_input("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(part1(input.clone()), 37);
    assert_eq!(part2(input), 168);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 331067);
    assert_eq!(part2(input), 92881128);
}
