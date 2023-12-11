use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector2> {
    parse_map_set(input)
}

fn default_input() -> HashSet<Vector2> {
    parse_input(include_input!(2023 / 11))
}

fn solve(space: HashSet<Vector2>, age: usize) -> usize {
    let (min_x, max_x) = space.iter().map(|v| v.x).min_max().unwrap();
    let (min_y, max_y) = space.iter().map(|v| v.y).min_max().unwrap();

    let x_expands: Vec<_> = (min_x..=max_x)
        .filter(|&x| (min_y..=max_y).all(|y| !space.contains(&vector![x, y])))
        .collect();
    let y_expands: Vec<_> = (min_y..=max_y)
        .filter(|&y| (min_x..=max_x).all(|x| !space.contains(&vector![x, y])))
        .collect();

    space
        .into_iter()
        .array_combinations()
        .map(|[g1, g2]| {
            let d = (g2 - g1).l1_norm() as usize;
            let x_expansion = x_expands
                .iter()
                .filter(|&&ex| (g1.x < ex && ex < g2.x) || (g2.x < ex && ex < g1.x))
                .count();
            let y_expansion = y_expands
                .iter()
                .filter(|&&ey| (g1.y < ey && ey < g2.y) || (g2.y < ey && ey < g1.y))
                .count();
            d + (age - 1) * (x_expansion + y_expansion)
        })
        .sum()
}

fn part1(space: HashSet<Vector2>) -> usize {
    solve(space, 2)
}

fn part2(space: HashSet<Vector2>) -> usize {
    solve(space, 1_000_000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
    );
    assert_eq!(part1(input.clone()), 374);
    assert_eq!(solve(input, 100), 8410);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 9693756);
    assert_eq!(part2(input), 717878258016);
}
