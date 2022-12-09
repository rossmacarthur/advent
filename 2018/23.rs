use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector3, i64)> {
    input
        .lines()
        .map(|line| {
            let caps = regex!(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")
                .captures(line)
                .unwrap();
            let x = caps[1].parse().unwrap();
            let y = caps[2].parse().unwrap();
            let z = caps[3].parse().unwrap();
            let r = caps[4].parse().unwrap();
            (vector![x, y, z], r)
        })
        .collect()
}

fn default_input() -> Vec<(Vector3, i64)> {
    parse_input(include_str!("input/23.txt"))
}

fn part1(nanobots: Vec<(Vector3, i64)>) -> usize {
    let (n, r) = *nanobots.iter().max_by_key(|(_, r)| r).unwrap();
    nanobots
        .iter()
        .filter(|(p, _)| (p - n).l1_norm() <= r)
        .count()
}

fn part2(nanobots: Vec<(Vector3, i64)>) -> i64 {
    let mut transitions = Vec::new();
    for (p, r) in nanobots {
        let d = p.x.abs() + p.y.abs() + p.z.abs();
        transitions.push((d - r, 1));
        transitions.push((d + r, -1));
    }
    transitions.sort_by_key(|(d, _)| *d);

    let mut result = 0;
    let mut count = 0;
    let mut max_count = 0;
    for (d, e) in transitions {
        count += e;
        if count > max_count {
            result = d;
            max_count = count;
        }
    }
    result
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1",
    );
    assert_eq!(part1(input), 7);
}

#[test]
fn example2() {
    let input = parse_input(
        r"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5",
    );
    assert_eq!(part2(input), 36);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 674);
    assert_eq!(part2(input), 129444177);
}
