use advent::prelude::*;

fn parse_input(input: &str) -> Target {
    let caps = regex!(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")
        .captures(input)
        .unwrap();
    let x0 = caps[1].parse().unwrap();
    let x1 = caps[2].parse().unwrap();
    let y0 = caps[3].parse().unwrap();
    let y1 = caps[4].parse().unwrap();
    Target { x0, x1, y0, y1 }
}

fn default_input() -> Target {
    parse_input("target area: x=155..182, y=-117..-67")
}

#[derive(Debug, Clone, Copy)]
struct Target {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

impl Target {
    fn contains(&self, p: Vector2) -> bool {
        let &Target { x0, x1, y0, y1 } = self;
        p.x >= x0 && p.x <= x1 && p.y >= y0 && p.y <= y1
    }
}

fn launch(mut v: Vector2, t: Target) -> Option<i64> {
    let mut maxy = 0;
    let mut p = vector![0, 0];
    while p.y >= t.y0 {
        p += v;
        v.x -= v.x.signum();
        v.y -= 1;
        if p.y > maxy {
            maxy = p.y;
        }
        if t.contains(p) {
            return Some(maxy);
        }
    }
    None
}

fn part1(target: Target) -> i64 {
    (1..=target.x1)
        .cartesian_product(0..=target.y0.abs())
        .filter_map(|(x, y)| launch(vector![x, y], target))
        .max()
        .unwrap()
}

fn part2(target: Target) -> usize {
    (1..=target.x1)
        .cartesian_product(target.y0..=target.y0.abs())
        .filter_map(|(x, y)| launch(vector![x, y], target))
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input("target area: x=20..30, y=-10..-5");
    assert_eq!(part1(input), 45);
    assert_eq!(part2(input), 112);
}

#[test]
fn example2() {
    let input = parse_input("target area: x=352..377, y=-49..-30");
    assert_eq!(part1(input), 66);
    assert_eq!(part2(input), 820);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6786);
    assert_eq!(part2(input), 2313);
}
