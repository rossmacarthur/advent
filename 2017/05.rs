use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2017 / 05))
}

fn solve(mut maze: Vec<i64>, f: impl Fn(&mut i64)) -> i64 {
    let mut steps = 0;
    let mut pos = 0;
    while let Some(jump) = maze.get_mut(pos as usize) {
        pos += *jump;
        f(jump);
        steps += 1;
    }
    steps
}

fn part1(maze: Vec<i64>) -> i64 {
    solve(maze, |j| *j += 1)
}

fn part2(maze: Vec<i64>) -> i64 {
    solve(maze, |j| *j += if *j >= 3 { -1 } else { 1 })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
0
3
0
1
-3",
    );
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 354121);
    assert_eq!(part2(input), 27283023);
}
