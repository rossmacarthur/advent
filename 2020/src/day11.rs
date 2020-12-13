use std::collections::HashMap;

const INPUT: &str = include_str!("input/day11.txt");

const VECTORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

type Point = (i32, i32);

type Grid = HashMap<Point, Position>;

pub fn default_input() -> Grid {
    INPUT
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .map(|c| match c {
                    '.' => Position::Floor,
                    'L' => Position::EmptySeat,
                    '#' => Position::OccupiedSeat,
                    _ => panic!("unexpected state"),
                })
                .enumerate()
                .map(move |(j, pos)| ((i as i32, j as i32), pos))
        })
        .collect()
}

/// Builds a visibility map from the grid.
fn visibility(grid: &Grid) -> HashMap<Point, Vec<Point>> {
    let mut visible = HashMap::new();
    for &(row, col) in grid.keys() {
        for (i, j) in &VECTORS {
            let mut point = (row + i, col + j);
            while let Some(position) = grid.get(&point) {
                if let Position::Floor = position {
                    point = (point.0 + i, point.1 + j);
                } else {
                    visible
                        .entry((row, col))
                        .or_insert_with(Vec::new)
                        .push(point);
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
        .filter(|position| matches!(position, Position::OccupiedSeat))
        .count()
}

/// Returns the number of adjacent occupied seats.
fn adjacent_occupied(grid: &Grid, point: (i32, i32)) -> usize {
    VECTORS
        .iter()
        .filter_map(|(i, j)| grid.get(&(point.0 + i, point.1 + j)))
        .filter(|position| matches!(position, Position::OccupiedSeat))
        .count()
}

/// Returns the number of visible occupied seats.
fn visible_occupied(grid: &Grid, visible: &HashMap<Point, Vec<Point>>, point: (i32, i32)) -> usize {
    visible[&point]
        .iter()
        .map(|point| &grid[point])
        .filter(|position| matches!(position, Position::OccupiedSeat))
        .count()
}

pub fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    loop {
        let mut next = grid.clone();
        for (&point, &position) in &grid {
            next.insert(
                point,
                match position {
                    Position::EmptySeat if adjacent_occupied(&grid, point) == 0 => {
                        Position::OccupiedSeat
                    }
                    Position::OccupiedSeat if adjacent_occupied(&grid, point) >= 4 => {
                        Position::EmptySeat
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

pub fn part2(grid: &Grid) -> usize {
    let visible = visibility(grid);
    let mut grid = grid.clone();
    loop {
        let mut next = grid.clone();
        for (&point, &position) in &grid {
            next.insert(
                point,
                match position {
                    Position::EmptySeat if visible_occupied(&grid, &visible, point) == 0 => {
                        Position::OccupiedSeat
                    }
                    Position::OccupiedSeat if visible_occupied(&grid, &visible, point) >= 5 => {
                        Position::EmptySeat
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
