use advent::prelude::*;

use Ordering::*;

fn parse_input(input: &str) -> Vec<[Vector2; 2]> {
    regex!(r"p=(\-?\d+),(\-?\d+) v=(\-?\d+),(\-?\d+)")
        .captures_iter(input)
        .map(|caps| {
            let px = caps[1].parse().unwrap();
            let py = caps[2].parse().unwrap();
            let vx = caps[3].parse().unwrap();
            let vy = caps[4].parse().unwrap();
            [vector![px, py], vector![vx, vy]]
        })
        .collect()
}

fn default_input() -> (Vec<[Vector2; 2]>, Vector2) {
    (parse_input(include_input!(2024 / 14)), vector![101, 103])
}

fn safety_factor(robots: &[[Vector2; 2]], max: Vector2, t: i64) -> i64 {
    robots
        .iter()
        .map(|[p, v]| {
            let q = p + v * t;
            vector![q.x.rem_euclid(max.x), q.y.rem_euclid(max.y)]
        })
        .fold([0, 0, 0, 0], |mut acc, p| {
            let mid = max / 2;
            match (p.x.cmp(&mid.x), p.y.cmp(&mid.y)) {
                (Less, Less) => acc[0] += 1,
                (Less, Greater) => acc[1] += 1,
                (Greater, Less) => acc[2] += 1,
                (Greater, Greater) => acc[3] += 1,
                _ => {}
            }
            acc
        })
        .into_iter()
        .product()
}

fn part1((robots, max): (Vec<[Vector2; 2]>, Vector2)) -> i64 {
    safety_factor(&robots, max, 100)
}

fn part2((robots, max): (Vec<[Vector2; 2]>, Vector2)) -> i64 {
    (0..max.x * max.y)
        .map(|t| (t, safety_factor(&robots, max, t)))
        .min_by_key(|(_, sf)| *sf)
        .map(|(t, _)| t)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    );
    assert_eq!(part1((input, vector![11, 7])), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 224438715);
    assert_eq!(part2(input), 7603);
}
