use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let as_binary: String = line
                .chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    c => panic!("invalid character `{c}`"),
                })
                .collect();
            assert_eq!(as_binary.len(), 10);
            let row = i64::from_str_radix(&as_binary[..7], 2).unwrap();
            let col = i64::from_str_radix(&as_binary[7..], 2).unwrap();
            row * 8 + col
        })
        .collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/05.txt"))
}

fn part1(ids: Vec<i64>) -> i64 {
    ids.into_iter().max().unwrap()
}

fn part2(mut ids: Vec<i64>) -> i64 {
    ids.sort_unstable();
    for [curr, next] in ids.into_iter().array_windows() {
        if next - curr > 1 {
            return next - 1;
        }
    }
    unreachable!()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL",
    );
    assert_eq!(part1(input), 820);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 883);
    assert_eq!(part2(input), 532);
}
