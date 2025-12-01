use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let sign = match &line[0..1] {
                "L" => -1,
                "R" => 1,
                s => panic!("unexpected direction: `{}`", s),
            };
            let rot = line[1..].parse().unwrap();
            (sign, rot)
        })
        .collect()
}

fn default_input() -> Vec<(i64, i64)> {
    parse_input(include_input!(2025 / 01))
}

fn part1(rotations: Vec<(i64, i64)>) -> i64 {
    let mut dial = 50;
    let mut pass = 0;
    for (sign, rot) in rotations {
        dial = (dial + sign * rot) % 100;
        if dial == 0 {
            pass += 1;
        }
    }
    pass
}

fn part2(rotations: Vec<(i64, i64)>) -> i64 {
    let mut dial = 50;
    let mut pass = 0;
    for (sign, rot) in rotations {
        for _ in 0..rot {
            dial = (dial + sign) % 100;
            if dial == 0 {
                pass += 1;
            }
        }
    }
    pass
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    );
    assert_eq!(part1(input.clone()), 3);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1023);
    assert_eq!(part2(input), 5899);
}
