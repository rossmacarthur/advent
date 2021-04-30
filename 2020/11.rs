use std::collections::HashMap;

use vectrix::{parse_map, vector, Vector2};

type Vector = Vector2<i64>;
type Grid = HashMap<Vector, Tile>;
type Visible = HashMap<Vector, Vec<Vector>>;

fn parse_input(input: &str) -> Grid {
    parse_map(input, |c| match c {
        '.' => Tile::Floor,
        'L' => Tile::EmptySeat,
        '#' => Tile::OccupiedSeat,
        _ => panic!("unexpected character"),
    })
}

fn default_input() -> Grid {
    parse_input(include_str!("input/11.txt"))
}

const DIRECTIONS: [Vector; 8] = [
    vector![-1, -1],
    vector![-1, 0],
    vector![-1, 1],
    vector![0, -1],
    vector![0, 1],
    vector![1, -1],
    vector![1, 0],
    vector![1, 1],
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

/// Builds a visibility map from the grid.
fn visibility(grid: &Grid) -> Visible {
    let mut visible = HashMap::new();
    for center in grid.keys() {
        for direction in &DIRECTIONS {
            let mut location = center + direction;
            while let Some(tile) = grid.get(&location) {
                if let Tile::Floor = tile {
                    location += direction;
                } else {
                    visible
                        .entry(*center)
                        .or_insert_with(Vec::new)
                        .push(location);
                    break;
                }
            }
        }
    }
    visible
}

/// Returns the number of occupied seats for a grid.
fn occupied(grid: &Grid) -> usize {
    grid.values()
        .filter(|tile| matches!(tile, Tile::OccupiedSeat))
        .count()
}

/// Returns the number of adjacent occupied seats.
fn adjacent_occupied(grid: &Grid, center: Vector) -> usize {
    DIRECTIONS
        .iter()
        .filter_map(|direction| grid.get(&(center + direction)))
        .filter(|tile| matches!(tile, Tile::OccupiedSeat))
        .count()
}

/// Returns the number of visible occupied seats.
fn visible_occupied(grid: &Grid, visible: &Visible, center: Vector) -> usize {
    visible[&center]
        .iter()
        .map(|location| &grid[location])
        .filter(|tile| matches!(tile, Tile::OccupiedSeat))
        .count()
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    loop {
        let mut next = grid.clone();
        for (&location, &tile) in &grid {
            next.insert(
                location,
                match tile {
                    Tile::EmptySeat if adjacent_occupied(&grid, location) == 0 => {
                        Tile::OccupiedSeat
                    }
                    Tile::OccupiedSeat if adjacent_occupied(&grid, location) >= 4 => {
                        Tile::EmptySeat
                    }
                    _ => continue,
                },
            );
        }
        if grid == next {
            break;
        }
        grid = next;
    }
    occupied(&grid)
}

fn part2(grid: &Grid) -> usize {
    let visible = visibility(grid);
    let mut grid = grid.clone();
    loop {
        let mut next = grid.clone();
        for (&location, &tile) in &grid {
            next.insert(
                location,
                match tile {
                    Tile::EmptySeat if visible_occupied(&grid, &visible, location) == 0 => {
                        Tile::OccupiedSeat
                    }
                    Tile::OccupiedSeat if visible_occupied(&grid, &visible, location) >= 5 => {
                        Tile::EmptySeat
                    }
                    _ => continue,
                },
            );
        }
        if grid == next {
            break;
        }
        grid = next;
    }
    occupied(&grid)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
    );
    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 26);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 2254);
    assert_eq!(part2(&input), 2004);
}
