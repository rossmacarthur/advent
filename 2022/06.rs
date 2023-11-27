use advent::prelude::*;

fn default_input() -> &'static str {
    include_str!("input/06.txt")
}

fn detect(signal: &str, n: usize) -> usize {
    signal
        .as_bytes()
        .windows(n)
        .position(|w| HashSet::from_iter(w).len() == n)
        .map(|i| i + n)
        .unwrap()
}

fn part1(signal: &str) -> usize {
    detect(signal, 4)
}

fn part2(input: &str) -> usize {
    detect(input, 14)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn example2() {
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1343);
    assert_eq!(part2(input), 2193);
}
