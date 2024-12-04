use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, char> {
    parse_map(input, |c| c)
}

fn default_input() -> HashMap<Vector2, char> {
    parse_input(include_input!(2024 / 04))
}

const DIRECTIONS: [Vector2; 8] = vectors!(
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1]
);

const CORNERS: [Vector2; 4] = vectors!([-1, -1], [-1, 1], [1, -1], [1, 1]);

/// Count the number of times the word "XMAS" appears in the grid starting with
/// an X at the given position.
fn count_xmas(words: &HashMap<Vector2, char>, p: Vector2) -> usize {
    DIRECTIONS
        .into_iter()
        .filter(|&d| {
            (1..4)
                .filter_map(|dd| words.get(&(p + d * dd)).copied())
                .eq("MAS".chars())
        })
        .count()
}

/// Count the number of times a "MAS" in the shape of an X appears in the grid
/// starting with an A at the given position.
fn count_x_mas(words: &HashMap<Vector2, char>, p: Vector2) -> usize {
    [
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
        ['S', 'S', 'M', 'M'],
    ]
    .into_iter()
    .filter(|&pattern| {
        CORNERS
            .into_iter()
            .filter_map(|d| words.get(&(p + d)).copied())
            .eq(pattern)
    })
    .count()
}

fn part1(words: HashMap<Vector2, char>) -> usize {
    words
        .iter()
        .filter_map(|(&p, &c)| (c == 'X').some_with(|| count_xmas(&words, p)))
        .sum()
}

fn part2(words: HashMap<Vector2, char>) -> usize {
    words
        .iter()
        .filter_map(|(&p, &c)| (c == 'A').some_with(|| count_x_mas(&words, p)))
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    );
    assert_eq!(part1(input.clone()), 18);
    assert_eq!(part2(input), 9);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2557);
    assert_eq!(part2(input), 1854);
}
