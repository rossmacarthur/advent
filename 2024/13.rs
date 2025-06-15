use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[Vector2; 3]> {
    regex!(
        r"Button A: X\+(?P<ax>\d+), Y\+(?P<ay>\d+)
Button B: X\+(?P<bx>\d+), Y\+(?P<by>\d+)
Prize: X=(?P<px>\d+), Y=(?P<py>\d+)"
    )
    .captures_iter(input)
    .map(|m| {
        let ax = m["ax"].parse().unwrap();
        let ay = m["ay"].parse().unwrap();
        let bx = m["bx"].parse().unwrap();
        let by = m["by"].parse().unwrap();
        let px = m["px"].parse().unwrap();
        let py = m["py"].parse().unwrap();
        [vector![ax, ay], vector![bx, by], vector![px, py]]
    })
    .collect()
}

fn default_input() -> Vec<[Vector2; 3]> {
    parse_input(include_input!(2024 / 13))
}

/// For each machine we're given two buttons A and B, and a prize position.
///
/// So given this
///
///     Button A: X+94, Y+34
///     Button B: X+22, Y+67
///     Prize: X=8400, Y=5400
///
///     - The change in position of the claw when pressing button A is [94, 34],
///       this we call `da`.
///     - The change in position of the claw when pressing button B is [22, 67],
///       this we call `db`.
///     - The position of the prize is [8400, 5400], this we call `p`.
///
/// We want to find non-negative integers `a` and `b` such that the following
/// two equations hold:
///
///     a * da.x + b * db.x = p.x
///     a * da.y + b * db.y = p.y
///
fn solve([da, db, p]: [Vector2; 3]) -> Option<i64> {
    // Multiply the first equation by `da.y`, and the second by `da.x`:
    //   a * da.x * da.y + b * db.x * da.y = p.x * da.y
    //   a * da.y * da.x + b * db.y * da.x = p.y * da.x
    //
    // Subtract the second from the first:
    //   (b * db.x * da.y) - (b * db.y * da.x) = p.x * da.y - p.y * da.x
    //
    // Extract `b`:
    //   b * (db.x * da.y - db.y * da.x) = p.x * da.y - p.y * da.x
    //
    // Re-arranging gives us:
    let b = (p.x * da.y - p.y * da.x) / (db.x * da.y - db.y * da.x);

    // Now we can substitute `b` into the first equation:
    let a = (p.x - b * db.x) / da.x;

    // Finally, we should check that the solutions actually equal the prize
    // position, this ensures that both a and b are non-negative integers.
    if (a * da.x + b * db.x, a * da.y + b * db.y) != (p.x, p.y) {
        return None;
    }

    Some(a * 3 + b)
}

fn part1(machines: Vec<[Vector2; 3]>) -> i64 {
    machines.into_iter().filter_map(solve).sum()
}

fn part2(machines: Vec<[Vector2; 3]>) -> i64 {
    machines
        .into_iter()
        .map(|[da, db, p]| [da, db, vector![p.x + 10000000000000, p.y + 10000000000000]])
        .filter_map(solve)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    );
    assert_eq!(part1(input.clone()), 480);
    assert_eq!(part2(input), 875318608908);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 33427);
    assert_eq!(part2(input), 91649162972270);
}
