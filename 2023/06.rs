use advent::prelude::*;

fn parse_line(line: &str) -> impl Iterator<Item = i64> + '_ {
    let (_, nums) = line.split_once(':').unwrap();
    let n = nums.replace(char::is_whitespace, "").parse().unwrap();
    iter::once(n).chain(nums.split_whitespace().map(str::parse).map(Result::unwrap))
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let [times, distances] = input.lines().collect_array();
    parse_line(times).zip(parse_line(distances)).collect()
}

fn default_input() -> Vec<(i64, i64)> {
    parse_input(include_input!(2023 / 06))
}

fn count((time, distance): (i64, i64)) -> usize {
    (0..time).filter(|t| (time - t) * t > distance).count()
}

fn part1(input: Vec<(i64, i64)>) -> usize {
    input.into_iter().skip(1).map(count).product()
}

fn part2(input: Vec<(i64, i64)>) -> usize {
    input.into_iter().take(1).map(count).product()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Time:      7  15   30
Distance:  9  40  200",
    );
    assert_eq!(part1(input.clone()), 288);
    assert_eq!(part2(input), 71503);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1624896);
    assert_eq!(part2(input), 32583852);
}
