use std::f64::consts;

use advent::prelude::*;

fn default_input() -> HashSet<Vector2> {
    parse_map_set(include_str!("input/10.txt"))
}

fn reduced(v: Vector2) -> Vector2 {
    let div = gcd(v.x, v.y);
    vector![v.x / div, v.y / div]
}

fn visible(asteroids: &HashSet<Vector2>, center: Vector2) -> HashMap<Vector2, Vec<(i64, Vector2)>> {
    let mut visible = HashMap::new();
    for a in asteroids.iter().copied() {
        if a == center {
            continue;
        }
        let dv = a - center;
        let element = (dv.l1_norm(), a);
        let vec = visible.entry(reduced(dv)).or_insert_with(Vec::new);
        let pos = vec.binary_search(&element).unwrap_err();
        vec.insert(pos, element);
    }
    visible
}

fn part1(asteroids: HashSet<Vector2>) -> usize {
    asteroids
        .iter()
        .copied()
        .map(|a| visible(&asteroids, a).len())
        .max()
        .unwrap()
}

fn part2(asteroids: HashSet<Vector2>) -> i64 {
    let visible = asteroids
        .iter()
        .copied()
        .map(|a| visible(&asteroids, a))
        .max_by_key(|visible| visible.len())
        .unwrap();
    visible
        .into_iter()
        .map(|(v, mut asteroids)| {
            let (_, a) = asteroids.remove(0);
            let angle = (v.x as f64).atan2(-v.y as f64).rem_euclid(consts::TAU);
            (angle, a)
        })
        .sorted_by(|(angle1, _), (angle2, _)| angle1.partial_cmp(angle2).unwrap())
        .nth(199)
        .map(|(_, a)| a.x * 100 + a.y)
        .unwrap()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_map_set(
        r#".#..#
.....
#####
....#
...##"#,
    );
    assert_eq!(part1(input), 8);
}

#[test]
fn example2() {
    let input = parse_map_set(
        r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#,
    );
    assert_eq!(part1(input), 33);
}

#[test]
fn example3() {
    let input = parse_map_set(
        r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#,
    );
    assert_eq!(part1(input), 35);
}

#[test]
fn example4() {
    let input = parse_map_set(
        r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#,
    );
    assert_eq!(part1(input), 41);
}

#[test]
fn example5() {
    let input = parse_map_set(
        r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#,
    );
    assert_eq!(part1(input.clone()), 210);
    assert_eq!(part2(input), 802);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 319);
    assert_eq!(part2(input), 517);
}
