use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use vectrix::{parse_map_set, Vector2, VectorExt};

type Vector = Vector2<i64>;

fn default_input() -> HashSet<Vector> {
    parse_map_set(include_str!("input/10.txt"))
}

fn visible(asteroids: &HashSet<Vector>, center: Vector) -> HashMap<Vector, Vec<(i64, Vector)>> {
    let mut visible = HashMap::new();
    for asteroid in asteroids.iter().copied() {
        if asteroid == center {
            continue;
        }
        let dv = asteroid - center;
        let element = (dv.l1_norm(), asteroid);
        let vec = visible.entry(dv.reduced()).or_insert_with(Vec::new);
        let pos = vec.binary_search(&element).unwrap_err();
        vec.insert(pos, element);
    }
    visible
}

fn part1(asteroids: &HashSet<Vector>) -> usize {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| visible(asteroids, asteroid).len())
        .max()
        .unwrap()
}

fn part2(asteroids: &HashSet<Vector>) -> i64 {
    asteroids
        .iter()
        .copied()
        .map(|asteroid| (visible(asteroids, asteroid), asteroid))
        .max_by_key(|(visible, _)| visible.len())
        .unwrap()
        .0
        .into_iter()
        .map(|(vector, mut asteroids)| (vector.rotated(90).angle(), asteroids.remove(0).1))
        .sorted_by(|(angle1, _), (angle2, _)| angle1.partial_cmp(angle2).unwrap())
        .nth(199)
        .map(|(_, asteroid)| asteroid.x * 100 + asteroid.y)
        .unwrap()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
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
    assert_eq!(part1(&input), 8);
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
    assert_eq!(part1(&input), 33);
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
    assert_eq!(part1(&input), 35);
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
    assert_eq!(part1(&input), 41);
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
    assert_eq!(part1(&input), 210);
    assert_eq!(part2(&input), 802);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 319);
    assert_eq!(part2(&input), 517);
}
