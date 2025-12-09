use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vector2> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap()
        })
        .map(Vector2::from)
        .collect()
}

fn default_input() -> Vec<Vector2> {
    parse_input(include_input!(2025 / 09))
}

fn part1(points: Vec<Vector2>) -> i64 {
    points
        .into_iter()
        .array_combinations()
        .map(|[p, q]| {
            let d = p - q;
            (d.x.abs() + 1) * (d.y.abs() + 1)
        })
        .max()
        .unwrap()
}

fn part2(points: Vec<Vector2>) -> i64 {
    points
        .iter()
        .copied()
        .array_combinations()
        .filter(|[p, q]| {
            // Get the rectangle defined by p and q
            let r1 = vector![min(p.x, q.x), min(p.y, q.y)];
            let r2 = vector![max(p.x, q.x), max(p.y, q.y)];
            points
                .iter()
                .copied()
                .circular_array_windows()
                .all(|[v1, v2]| {
                    let e1 = vector![min(v1.x, v2.x), min(v1.y, v2.y)];
                    let e2 = vector![max(v1.x, v2.x), max(v1.y, v2.y)];
                    // Check edge does not overlap with rectangle
                    e2.x <= r1.x || r2.x <= e1.x || e2.y <= r1.y || r2.y <= e1.y
                })
        })
        .map(|[p, q]| {
            let d = p - q;
            (d.x.abs() + 1) * (d.y.abs() + 1)
        })
        .max()
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    );
    assert_eq!(part1(input.clone()), 50);
    assert_eq!(part2(input), 24);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4748769124);
    assert_eq!(part2(input), 1525991432);
}
