use advent::prelude::*;

type Image = HashMap<Vector2, bool>;

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let [alg, image] = input.split("\n\n").collect_array();
    let alg = alg.chars().map(|c| c == '#').collect();
    let image = parse_map(image, |c| c == '#');
    (alg, image)
}

fn default_input() -> (Vec<bool>, Image) {
    parse_input(include_input!(2021 / 20))
}

fn step(alg: &[bool], image: Image, is_odd: bool) -> Image {
    let [min_x, max_x, min_y, max_y] = image.iter().filter(|&(_, v)| *v).map(|(&p, _)| p).fold(
        [i64::MAX, i64::MIN, i64::MAX, i64::MIN],
        |acc, p| {
            let [min_x, max_x, min_y, max_y] = acc;
            [
                min(min_x, p.x),
                max(max_x, p.x),
                min(min_y, p.y),
                max(max_y, p.y),
            ]
        },
    );

    let points = ((min_x - 1)..=(max_x + 1))
        .cartesian_product((min_y - 1)..=(max_y + 1))
        .map(|(x, y)| vector![x, y]);

    points
        .map(|p| {
            let i = (-1..=1)
                .cartesian_product(-1..=1)
                .map(|(y, x)| vector![x, y])
                .fold(0, |acc, d| {
                    let b = image.get(&(p + d)).copied().unwrap_or(alg[0] && is_odd) as usize;
                    acc * 2 + b
                });
            (p, alg[i])
        })
        .collect()
}

fn enhance(alg: Vec<bool>, mut image: Image, n: usize) -> usize {
    for t in 0..n {
        image = step(&alg, image, t % 2 != 0);
    }
    image.values().filter(|v| **v).count()
}

fn part1((alg, image): (Vec<bool>, Image)) -> usize {
    enhance(alg, image, 2)
}

fn part2((alg, image): (Vec<bool>, Image)) -> usize {
    enhance(alg, image, 50)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###");
    assert_eq!(part1(input.clone()), 35);
    assert_eq!(part2(input), 3351);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5475);
    assert_eq!(part2(input), 17548);
}
