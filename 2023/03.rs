use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    parse_map(input, |c| match c {
        '.' => Tile::Empty,
        c if c.is_ascii_digit() => Tile::Digit(c as i64 - b'0' as i64),
        c => Tile::Symbol(c),
    })
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_input!(2023 / 03))
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Symbol(char),
    Digit(i64),
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

fn solve(engine: &HashMap<Vector2, Tile>) -> HashMap<(Vector2, char), Vec<i64>> {
    let (min_x, max_x) = engine.keys().map(|v| v.x).minmax().into_option().unwrap();
    let (min_y, max_y) = engine.keys().map(|v| v.y).minmax().into_option().unwrap();

    let mut symbols = HashMap::new();

    for y in min_y..=max_y {
        let (mut n, mut symbol) = (None, None);
        for v in (min_x..=max_x).map(|x| vector![x, y]) {
            if let Tile::Digit(x) = engine[&v] {
                n = n.map(|n| n * 10 + x).or(Some(x));
                symbol = symbol.or_else(|| {
                    DIRECTIONS
                        .iter()
                        .map(|dv| v + dv)
                        .find_map(|v| match engine.get(&v) {
                            Some(Tile::Symbol(symbol)) => Some((v, *symbol)),
                            _ => None,
                        })
                });
            } else {
                if let (Some(n), Some(symbol)) = (n, symbol) {
                    symbols.entry(symbol).or_insert_with(Vec::new).push(n);
                }
                (n, symbol) = (None, None);
            }
        }
        if let (Some(n), Some(symbol)) = (n, symbol) {
            symbols.entry(symbol).or_insert_with(Vec::new).push(n);
        }
    }

    symbols
}

fn part1(engine: HashMap<Vector2, Tile>) -> i64 {
    solve(&engine).into_values().flatten().sum()
}

fn part2(engine: HashMap<Vector2, Tile>) -> i64 {
    solve(&engine)
        .into_iter()
        .filter(|((_, symbol), nums)| *symbol == '*' && nums.len() == 2)
        .map(|(_, nums)| nums.into_iter().product::<i64>())
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );
    assert_eq!(part1(input.clone()), 4361);
    assert_eq!(part2(input), 467835);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 550064);
    assert_eq!(part2(input), 85010461);
}
