use std::collections::HashSet;

use vectrix::{parse_map_set, vector, Vector2};

const INPUT: &str = include_str!("input/03.txt");

type Vector = Vector2<i64>;

pub fn default_input() -> HashSet<Vector> {
    parse_map_set(INPUT)
}

fn count_trees_for_slope(map: &HashSet<Vector>, slope: Vector) -> usize {
    let len_x = map.iter().map(|v| v.x).max().unwrap() + 1;
    let len_y = map.iter().map(|v| v.y).max().unwrap() + 1;
    let mut trees = 0;
    let mut location = Vector::zero();
    while location.y < len_y {
        if map.contains(&location) {
            trees += 1;
        }
        location += slope;
        location.x %= len_x;
    }
    trees
}

pub fn part1(map: &HashSet<Vector>) -> usize {
    count_trees_for_slope(map, vector![3, 1])
}

pub fn part2(map: &HashSet<Vector>) -> usize {
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .copied()
        .map(Vector::from)
        .map(|slope| count_trees_for_slope(map, slope))
        .product()
}

#[test]
fn example() {
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
