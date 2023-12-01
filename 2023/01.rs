use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn part1(calibration: &str) -> i64 {
    calibration
        .lines()
        .map(|line| {
            let digits = line
                .bytes()
                .filter(u8::is_ascii_digit)
                .map(|b| (b - b'0') as i64);
            digits.clone().next().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn part2(calibration: &str) -> i64 {
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    calibration
        .lines()
        .map(|line| {
            let digits = line.bytes().enumerate().filter_map(|(i, b)| {
                if b.is_ascii_digit() {
                    return Some((b - b'0') as i64);
                }
                NUMBERS
                    .into_iter()
                    .enumerate()
                    .filter(|(_, n)| line[i..].starts_with(n))
                    .map(|(j, _)| (j + 1) as i64)
                    .next()
            });
            digits.clone().next().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input), 142);
    assert_eq!(part2(input), 142);
}

#[test]
fn example2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(input), 281);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 55130);
    assert_eq!(part2(input), 54985);
}
