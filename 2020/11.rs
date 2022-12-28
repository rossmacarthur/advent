use advent::prelude::*;

type Grid = HashMap<Vector2, Tile>;
type Visible = HashMap<Vector2, Vec<Vector2>>;

fn parse_input(input: &str) -> Grid {
    parse_map(input, |c| match c {
        '.' => Tile::Floor,
        'L' => Tile::EmptySeat,
        '#' => Tile::OccupiedSeat,
        c => panic!("unexpected character `{c}`"),
    })
}

fn default_input() -> Grid {
    parse_input(include_str!("input/11.txt"))
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Tile {
    fn is_occupied(&self) -> bool {
        matches!(*self, Tile::OccupiedSeat)
    }
}

/// Builds a visibility map from the grid.
fn visibility(grid: &Grid) -> Visible {
    let mut visible = HashMap::new();
    for k in grid.keys() {
        for direction in &DIRECTIONS {
            let mut p = k + direction;
            while let Some(tile) = grid.get(&p) {
                match tile {
                    Tile::Floor => p += direction,
                    _ => {
                        visible.entry(*k).or_insert_with(Vec::new).push(p);
                        break;
                    }
                }
            }
        }
    }
    visible
}

/// Returns the number of adjacent occupied seats.
fn adjacent_occupied(grid: &Grid, p: Vector2) -> usize {
    DIRECTIONS
        .iter()
        .filter_map(|d| grid.get(&(p + d)))
        .filter(|t| t.is_occupied())
        .count()
}

/// Returns the number of visible occupied seats.
fn visible_occupied(grid: &Grid, vis: &Visible, p: Vector2) -> usize {
    vis[&p]
        .iter()
        .map(|p| &grid[p])
        .filter(|t| t.is_occupied())
        .count()
}

fn solve<F>(mut grid: Grid, f: F) -> usize
where
    F: Fn(&Grid, Vector2, Tile) -> (Vector2, Tile) + Copy,
{
    loop {
        let next = grid.iter().map(|(&p, &t)| f(&grid, p, t)).collect();
        if grid == next {
            break grid.values().filter(|t| t.is_occupied()).count();
        }
        grid = next;
    }
}

fn part1(grid: Grid) -> usize {
    solve(grid, |grid, p, t| {
        let t = match t {
            Tile::EmptySeat if adjacent_occupied(grid, p) == 0 => Tile::OccupiedSeat,
            Tile::OccupiedSeat if adjacent_occupied(grid, p) >= 4 => Tile::EmptySeat,
            t => t,
        };
        (p, t)
    })
}

fn part2(grid: Grid) -> usize {
    let vis = visibility(&grid);
    solve(grid, |grid, p, t| {
        let t = match t {
            Tile::EmptySeat if visible_occupied(grid, &vis, p) == 0 => Tile::OccupiedSeat,
            Tile::OccupiedSeat if visible_occupied(grid, &vis, p) >= 5 => Tile::EmptySeat,
            t => t,
        };
        (p, t)
    })
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    );
    assert_eq!(part1(input.clone()), 37);
    assert_eq!(part2(input), 26);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2254);
    assert_eq!(part2(input), 2004);
}
