use advent::prelude::*;

fn default_input() -> HashSet<Vector2> {
    parse_map_set(include_input!(2025 / 04))
}

const DIRECTIONS: [Vector2; 8] = vectors!(
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
);

fn part1(rolls: HashSet<Vector2>) -> usize {
    rolls
        .iter()
        .filter(|&roll| {
            DIRECTIONS
                .iter()
                .map(|d| roll + d)
                .filter(|p| rolls.contains(p))
                .count()
                < 4
        })
        .count()
}

fn part2(rolls: HashSet<Vector2>) -> usize {
    let mut removed: HashSet<Vector2> = HashSet::new();
    loop {
        let more: Vec<_> = rolls
            .iter()
            .filter(|&roll| {
                !removed.contains(roll)
                    && DIRECTIONS
                        .iter()
                        .map(|d| roll + d)
                        .filter(|p| !removed.contains(p) && rolls.contains(p))
                        .count()
                        < 4
            })
            .collect();
        if more.is_empty() {
            return removed.len();
        }
        removed.extend(more);
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_map_set(
        "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    );
    assert_eq!(part1(input.clone()), 13);
    assert_eq!(part2(input), 43);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1367);
    assert_eq!(part2(input), 9144);
}
