use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2)> {
    regex!(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
        .captures_iter(input)
        .map(|caps| {
            let px = caps[1].parse().unwrap();
            let py = caps[2].parse().unwrap();
            let vx = caps[3].parse().unwrap();
            let vy = caps[4].parse().unwrap();
            (vector![px, py], vector![vx, vy])
        })
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2)> {
    parse_input(include_str!("input/10.txt"))
}

fn to_string(points: &[(Vector2, Vector2)]) -> String {
    let min_x = points.iter().map(|(p, _)| p.x).min().unwrap();
    let min_y = points.iter().map(|(p, _)| p.y).min().unwrap();
    let max_x = points.iter().map(|(p, _)| p.x).max().unwrap();
    let max_y = points.iter().map(|(p, _)| p.y).max().unwrap();
    let set: HashSet<_> = points.iter().map(|(p, _)| *p).collect();
    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if set.contains(&vector!(x, y)) {
                s.push('█');
            } else {
                s.push('░');
            }
        }
        s.push('\n');
    }
    s
}

fn solve(mut points: Vec<(Vector2, Vector2)>) -> (Vec<(Vector2, Vector2)>, usize) {
    let mut secs = 0;
    let mut height = i64::MAX;
    loop {
        let min_y = points.iter().map(|(p, _)| p.y).min().unwrap();
        let max_y = points.iter().map(|(p, _)| p.y).max().unwrap();
        let h = max_y - min_y;
        if h > height {
            break;
        }
        height = h;
        secs += 1;
        for (p, v) in &mut points {
            *p += *v;
        }
    }
    for (p, v) in &mut points {
        *p -= *v;
    }
    (points, secs - 1)
}

fn part1(points: Vec<(Vector2, Vector2)>) -> String {
    let (points, _) = solve(points);
    to_string(&points)
}

fn part2(points: Vec<(Vector2, Vector2)>) -> usize {
    let (_, secs) = solve(points);
    secs
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>",
    );
    assert_eq!(
        part1(input.clone()),
        "\
█░░░█░░███
█░░░█░░░█░
█░░░█░░░█░
█████░░░█░
█░░░█░░░█░
█░░░█░░░█░
█░░░█░░░█░
█░░░█░░███
"
    );
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(
        part1(input.clone()),
        "\
█░░░░█░░█░░░░░░░██████░░░░██░░░░█░░░░█░░█████░░░░████░░░██████
█░░░░█░░█░░░░░░░░░░░░█░░░█░░█░░░█░░░█░░░█░░░░█░░█░░░░█░░░░░░░█
░█░░█░░░█░░░░░░░░░░░░█░░█░░░░█░░█░░█░░░░█░░░░█░░█░░░░░░░░░░░░█
░█░░█░░░█░░░░░░░░░░░█░░░█░░░░█░░█░█░░░░░█░░░░█░░█░░░░░░░░░░░█░
░░██░░░░█░░░░░░░░░░█░░░░█░░░░█░░██░░░░░░█████░░░█░░░░░░░░░░█░░
░░██░░░░█░░░░░░░░░█░░░░░██████░░██░░░░░░█░░░░█░░█░░███░░░░█░░░
░█░░█░░░█░░░░░░░░█░░░░░░█░░░░█░░█░█░░░░░█░░░░█░░█░░░░█░░░█░░░░
░█░░█░░░█░░░░░░░█░░░░░░░█░░░░█░░█░░█░░░░█░░░░█░░█░░░░█░░█░░░░░
█░░░░█░░█░░░░░░░█░░░░░░░█░░░░█░░█░░░█░░░█░░░░█░░█░░░██░░█░░░░░
█░░░░█░░██████░░██████░░█░░░░█░░█░░░░█░░█████░░░░███░█░░██████
"
    );
    assert_eq!(part2(input), 10656);
}
