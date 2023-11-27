use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, i64)> {
    input
        .lines()
        .map(|line| {
            let (d, n) = line.split_once(' ').unwrap();
            let d = match d {
                "U" => vector![0, 1],
                "D" => vector![0, -1],
                "L" => vector![-1, 0],
                "R" => vector![1, 0],
                d => panic!("unknown direction `{d}`"),
            };
            let n = n.parse::<i64>().unwrap();
            (d, n)
        })
        .collect()
}

fn default_input() -> Vec<(Vector2, i64)> {
    parse_input(include_str!("input/09.txt"))
}

fn solve<const N: usize>(cmds: Vec<(Vector2, i64)>) -> usize {
    let mut rope = [Vector2::zero(); N];
    let mut visited = HashSet::from_iter([rope[N - 1]]);
    for (dh, n) in cmds {
        for _ in 0..n {
            // Move the head of the rope
            rope[0] += dh;
            // Now move each knot in turn
            for i in 1..N {
                let dt = rope[i - 1] - rope[i];
                if dt.x.abs() > 1 || dt.y.abs() > 1 {
                    rope[i].x += dt.x.signum();
                    rope[i].y += dt.y.signum();
                }
            }
            // Insert the tail
            visited.insert(rope[N - 1]);
        }
    }
    visited.len()
}

fn part1(cmds: Vec<(Vector2, i64)>) -> usize {
    solve::<2>(cmds)
}

fn part2(cmds: Vec<(Vector2, i64)>) -> usize {
    solve::<10>(cmds)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    );
    assert_eq!(part1(input), 13);
}

#[test]
fn example2() {
    let input = parse_input(
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    );
    assert_eq!(part2(input), 36);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6470);
    assert_eq!(part2(input), 2658);
}
