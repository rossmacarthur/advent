use advent::prelude::*;

fn parse_point(input: &str) -> Vector2 {
    input
        .split(", ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect_array()
        .into()
}

fn parse_input(input: &str) -> Vec<Vector2> {
    input.lines().map(parse_point).collect()
}

fn default_input() -> Vec<Vector2> {
    parse_input(include_input!(2018 / 06))
}

fn part1(points: Vec<Vector2>) -> usize {
    let (min_x, max_x) = points.iter().map(|p| p.x).min_max().unwrap();
    let (min_y, max_y) = points.iter().map(|p| p.y).min_max().unwrap();

    let mut infinites = HashSet::new();
    let mut counts = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let loc = vector![x, y];
            let mut closest = None;
            let mut dist = i64::MAX;
            for p in &points {
                let d = (loc - p).l1_norm();
                match d.cmp(&dist) {
                    Ordering::Less => {
                        closest = Some(p);
                        dist = d;
                    }
                    Ordering::Equal => {
                        closest = None;
                    }
                    Ordering::Greater => {}
                }
            }
            if let Some(closest) = closest {
                *counts.entry(*closest).or_default() += 1;
                if loc.x == min_x || loc.x == max_x || loc.y == min_y || loc.y == max_y {
                    infinites.insert(*closest);
                }
            }
        }
    }

    counts
        .into_iter()
        .filter(|(p, _)| !infinites.contains(p))
        .map(|(_, count)| count)
        .max()
        .unwrap()
}

fn part2(points: Vec<Vector2>, limit: i64) -> i64 {
    let (min_x, max_x) = points.iter().map(|p| p.x).min_max().unwrap();
    let (min_y, max_y) = points.iter().map(|p| p.y).min_max().unwrap();

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let loc = vector![x, y];
            let d: i64 = points.iter().map(|&p| (loc - p).l1_norm()).sum();
            if d < limit {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let solution = advent::new(default_input)
        .part(part1)
        .part(|i| part2(i, 10_000))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
    );
    assert_eq!(part1(input.clone()), 17);
    assert_eq!(part2(input, 32), 16);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4589);
    assert_eq!(part2(input, 10_000), 40252);
}
