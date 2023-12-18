use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2)> {
    input
        .lines()
        .map(|line| {
            let re = regex!(r"(?P<dir>[URDL]) (?P<val>\d+) \(#(?P<color>[0-9a-f]{6})\)");
            let caps = re.captures(line).unwrap();

            let dp1 = {
                let d = match &caps["dir"] {
                    "U" => UP,
                    "R" => RIGHT,
                    "D" => DOWN,
                    "L" => LEFT,
                    _ => unreachable!(),
                };
                let v: i64 = caps["val"].parse().unwrap();
                d * v
            };

            let dp2 = {
                let color = i64::from_str_radix(&caps["color"], 16).unwrap();
                let d = match color % 16 {
                    0 => RIGHT,
                    1 => DOWN,
                    2 => LEFT,
                    3 => UP,
                    d => panic!("invalid direction `{d}`"),
                };
                let v = color / 16;
                d * v
            };

            (dp1, dp2)
        })
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2)> {
    parse_input(include_input!(2023 / 18))
}

const UP: Vector2 = vector![0, 1];
const RIGHT: Vector2 = vector![1, 0];
const DOWN: Vector2 = vector![0, -1];
const LEFT: Vector2 = vector![-1, 0];

fn solve<I>(plan: I) -> i64
where
    I: Iterator<Item = Vector2>,
{
    let vertices: Vec<_> = plan
        .into_iter()
        .scan(vector![0, 0], |p, dp| {
            *p += dp;
            Some(*p)
        })
        .collect();

    let mut s = 0;
    let mut b = 0;
    for [p, q] in vertices.into_iter().circular_array_windows() {
        s += p.x * q.y - q.x * p.y;
        b += (q - p).l1_norm();
    }
    (s.abs() + b) / 2 + 1
}

fn part1(plan: Vec<(Vector2, Vector2)>) -> i64 {
    solve(plan.into_iter().map(|(dp, _)| dp))
}

fn part2(plan: Vec<(Vector2, Vector2)>) -> i64 {
    solve(plan.into_iter().map(|(_, dp)| dp))
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    );
    assert_eq!(part1(input.clone()), 62);
    assert_eq!(part2(input), 952408144115);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 48503);
    assert_eq!(part2(input), 148442153147147);
}
