use std::collections::HashSet;

use vector::i64::xy::Vector;

use crate::map::parse_map_set;

const INPUT: &str = include_str!("input/day03.txt");

pub fn default_input() -> HashSet<Vector> {
    parse_map_set(INPUT)
}

fn count_trees_for_slope(map: &HashSet<Vector>, slope: Vector) -> usize {
    let len_x = map.iter().map(Vector::x).max().unwrap() + 1;
    let len_y = map.iter().map(Vector::y).max().unwrap() + 1;
    let mut trees = 0;
    let mut location = Vector::zero();
    while location.y() < len_y {
        if map.contains(&location) {
            trees += 1;
        }
        location += slope;
        *location.x_mut() %= len_x;
    }
    trees
}

pub fn part1(map: &HashSet<Vector>) -> usize {
    count_trees_for_slope(map, Vector::new([3, 1]))
}

pub fn part2(map: &HashSet<Vector>) -> usize {
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .copied()
        .map(Vector::new)
        .map(|slope| count_trees_for_slope(map, slope))
        .product()
}

#[test]
fn ex1() {
    let input = parse_map_set(
        r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#,
    );
    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 336);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 282);
    assert_eq!(part2(&input), 958815792);
}
