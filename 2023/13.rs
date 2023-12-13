use advent::prelude::*;

fn parse_input(input: &str) -> Vec<HashSet<Vector2>> {
    input.split("\n\n").map(parse_map_set).collect()
}

fn default_input() -> Vec<HashSet<Vector2>> {
    parse_input(include_input!(2023 / 13))
}

fn score(pattern: HashSet<Vector2>, part2: bool) -> i64 {
    let exp = if part2 { 1 } else { 0 };
    let (min_x, max_x) = pattern.iter().map(|v| v.x).min_max().unwrap();
    let (min_y, max_y) = pattern.iter().map(|v| v.y).min_max().unwrap();
    (min_x..max_x)
        .filter(|&x| {
            let smudged = (min_x..max_x)
                .map(|dx| (x - dx, x + dx + 1))
                .filter(|&(x0, x1)| min_x <= x0 && x1 <= max_x)
                .flat_map(|(x0, x1)| (min_y..=max_y).map(move |y| (vector![x0, y], vector![x1, y])))
                .filter(|&(l, r)| pattern.contains(&l) != pattern.contains(&r))
                .count();
            smudged == exp
        })
        .map(|x| x + 1)
        .sum()
}

fn score_reflection(pattern: HashSet<Vector2>, part2: bool) -> i64 {
    let transposed = pattern.iter().map(|v| vector![v.y, v.x]).collect();
    100 * score(transposed, part2) + score(pattern, part2)
}

fn part1(input: Vec<HashSet<Vector2>>) -> i64 {
    input
        .into_iter()
        .map(|pattern| score_reflection(pattern, false))
        .sum()
}

fn part2(input: Vec<HashSet<Vector2>>) -> i64 {
    input
        .into_iter()
        .map(|pattern| score_reflection(pattern, true))
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    );
    assert_eq!(part1(input.clone()), 405);
    assert_eq!(part2(input), 400);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 42974);
    assert_eq!(part2(input), 27587);
}
