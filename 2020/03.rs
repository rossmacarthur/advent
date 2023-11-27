use advent::prelude::*;

fn default_input() -> HashSet<Vector2> {
    parse_map_set(include_str!("input/03.txt"))
}

fn count_trees(map: &HashSet<Vector2>, slope: Vector2) -> usize {
    let len_x = map.iter().map(|v| v.x).max().unwrap() + 1;
    let len_y = map.iter().map(|v| v.y).max().unwrap() + 1;
    let mut trees = 0;
    let mut p = Vector2::zero();
    while p.y < len_y {
        if map.contains(&p) {
            trees += 1;
        }
        p += slope;
        p.x %= len_x;
    }
    trees
}

fn part1(map: HashSet<Vector2>) -> usize {
    count_trees(&map, vector![3, 1])
}

fn part2(map: HashSet<Vector2>) -> usize {
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .into_iter()
        .map(Vector2::from)
        .map(|slope| count_trees(&map, slope))
        .product()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_map_set(
        "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    );
    assert_eq!(part1(input.clone()), 7);
    assert_eq!(part2(input), 336);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 282);
    assert_eq!(part2(input), 958815792);
}
