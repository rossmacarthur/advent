use std::ops::{Deref, DerefMut};

use advent::prelude::*;

type Edge = [Pixel; 10];

fn parse_tile(grid: &str) -> Tile {
    let mut pixels = grid.lines().flat_map(|line| {
        line.chars().map(|c| match c {
            '.' => Pixel::Black,
            '#' => Pixel::White,
            c => panic!("unrecognized character `{c}`"),
        })
    });
    Tile([(); 10].map(|_| [(); 10].map(|_| pixels.next().unwrap())))
}

fn parse_input(input: &str) -> HashMap<i64, Tile> {
    regex!(r"Tile (?P<id>\d+):\n(?P<grid>[\n\.#]+)")
        .captures_iter(input)
        .map(|caps| {
            let id = caps["id"].parse().unwrap();
            let tile = parse_tile(&caps["grid"]);
            (id, tile)
        })
        .collect()
}

fn default_input() -> HashMap<i64, Tile> {
    parse_input(include_input!(2020 / 20))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Tile([[Pixel; 10]; 10]);

impl Default for Pixel {
    fn default() -> Self {
        Self::Black
    }
}

impl Deref for Tile {
    type Target = [[Pixel; 10]; 10];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Tile {
    fn top(&self) -> Edge {
        self[0]
    }
    fn bottom(&self) -> Edge {
        self[9]
    }
    fn left(&self) -> Edge {
        self.map(|row| row[0])
    }
    fn right(&self) -> Edge {
        self.map(|row| row[9])
    }

    fn edges(&self) -> [Edge; 4] {
        [self.top(), self.bottom(), self.left(), self.right()]
    }

    fn rotated(self) -> Tile {
        let mut tile = Tile([[Pixel::Black; 10]; 10]);
        for i in 0..10 {
            for j in 0..10 {
                tile[j][10 - 1 - i] = self[i][j]
            }
        }
        tile
    }

    fn flipped(mut self) -> Tile {
        self.reverse();
        self
    }
}

/// Returns all possible orientations of a tile.
fn orientations(tile: Tile) -> Vec<Tile> {
    let mut tiles = Vec::with_capacity(4 * 2);
    let mut temp = [tile, tile.flipped()];
    for _ in 0..4 {
        for t in &mut temp {
            tiles.push(*t);
            *t = t.rotated();
        }
    }
    tiles
}

/// Returns all possible edge permutations of a tile.
fn edges(tile: Tile) -> Vec<Edge> {
    let rev = |mut edge: Edge| {
        edge.reverse();
        edge
    };
    tile.edges().into_iter().flat_map(|e| [e, rev(e)]).collect()
}

/// Assembles the tiles into an image.
fn assemble(tiles: HashMap<i64, Tile>) -> HashSet<Vector2> {
    // For every edge possible list the tiles that could possibly have it.
    let edges = tiles.iter().fold(HashMap::new(), |acc, (&id, &tile)| {
        edges(tile).into_iter().fold(acc, |mut acc, edge| {
            acc.entry(edge).or_insert_with(HashSet::new).insert(id);
            acc
        })
    });

    // Start with an empty grid to place all the tiles.
    let d = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec![Tile::default(); d]; d];
    let mut placed = HashSet::new();

    // Most of the work happens in this macro: it searches through the given
    // tile ids and tries every orientation of the tile until it matches the
    // provided predicate. If it does then it places the tile at the point.
    macro_rules! place {
        (($i:expr, $j:expr), $ids:expr, where |$tile:ident| $pred:expr) => {{
            let (id, tile) = $ids
                .filter(|id| !placed.contains(*id))
                .flat_map(|id| orientations(tiles[id]).into_iter().map(move |t| (*id, t)))
                .find(|(_, $tile)| $pred)
                .unwrap();
            grid[$i][$j] = tile;
            placed.insert(id);
        }};
    }

    // An edge must be a border if there is only one tile that has it.
    let is_border = |t| edges[&t].len() == 1;

    // Find the first corner and place it in the top left.
    place! {
        (0, 0), tiles.keys(),
        where |tile| is_border(tile.left()) && is_border(tile.top())
    };

    // Now that we have the corner we can populate the rest of the first row
    // purely based on the tile to the left.
    let i = 0;
    for j in 1..d {
        let prev = grid[i][j - 1].right();
        place! {
            (i, j), edges[&prev].iter(),
            where |tile| tile.left() == prev && is_border(tile.top())
        };
    }

    // Now that we have the corner we can populate the rest of the first column
    // purely based on the tile to the top.
    let j = 0;
    for i in 1..d {
        let above = grid[i - 1][j].bottom();
        place! {
            (i, j), edges[&above].iter(),
            where |tile| tile.top() == above && is_border(tile.left())
        };
    }

    // Now that we have the first row and column we can populate the rest of the
    // rows purely based on the tile to the left and to the top.
    for (i, j) in iproduct!(1..d, 1..d) {
        let prev = grid[i][j - 1].right();
        let above = grid[i - 1][j].bottom();
        place! {
            (i, j), edges[&prev].iter(),
            where |tile| tile.left() == prev && tile.top() == above
        };
    }

    // Remove the borders and convert the tiles to a single image.
    iproduct!(0..d, 1..9)
        .enumerate()
        .flat_map(|(y, (r, i))| {
            let grid = &grid;
            iproduct!(0..d, 1..9)
                .enumerate()
                .filter_map(move |(x, (c, j))| match grid[r][c][i][j] {
                    Pixel::Black => None,
                    Pixel::White => Some(vector![x as i64, y as i64]),
                })
        })
        .collect()
}

fn flip(img: HashSet<Vector2>) -> HashSet<Vector2> {
    let max = img.iter().map(|v| v.x).max().unwrap();
    img.into_iter().map(|v| vector![max - v.x, v.y]).collect()
}

fn rotate(img: HashSet<Vector2>) -> HashSet<Vector2> {
    let max = img.iter().map(|v| v.x).max().unwrap();
    img.into_iter().map(|v| vector![v.y, max - v.x]).collect()
}

fn roughness(img: &HashSet<Vector2>, monster: &HashSet<Vector2>) -> Option<usize> {
    let mut rem = img.clone();
    let max_x = rem.iter().map(|v| v.x).max().unwrap();
    let max_y = rem.iter().map(|v| v.y).max().unwrap();
    for (x, y) in iproduct!(0..=max_x, 0..=max_y) {
        let d = vector![x, y];
        let monster: HashSet<_> = monster.iter().map(|p| p + d).collect();
        if monster.iter().all(|p| rem.contains(p)) {
            rem = rem.difference(&monster).copied().collect();
        }
    }
    (img.len() != rem.len()).some(rem.len())
}

fn part1(tiles: HashMap<i64, Tile>) -> i64 {
    let possible_edges: HashMap<_, HashSet<_>> = tiles
        .into_iter()
        .map(|(id, tile)| (id, HashSet::from_iter(edges(tile))))
        .collect();

    // Find all the tiles that have edges that only have two intersections with
    // another tile. These must be the corners.
    possible_edges
        .iter()
        .filter_map(|(&id, edges)| {
            let adjacents = possible_edges
                .iter()
                .filter_map(|(&other_id, other_edges)| {
                    (id != other_id && edges.intersection(other_edges).count() > 0).some(other_id)
                })
                .count();
            (adjacents == 2).some(id)
        })
        .product()
}

fn part2(tiles: HashMap<i64, Tile>) -> usize {
    // Firstly assemble the tiles into a single image.
    let img = assemble(tiles);

    // Now figure out the roughness. This might require flipping and rotating
    // the monster.
    let monster = parse_map_set(
        "\
..................#
#....##....##....###
.#..#..#..#..#..#",
    );
    let mut flips = [monster.clone(), flip(monster)];
    for _ in 0..4 {
        for m in &mut flips {
            match roughness(&img, m) {
                Some(r) => return r,
                None => *m = rotate(m.clone()),
            }
        }
    }
    panic!("no monsters found")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
",
    );
    assert_eq!(part1(input.clone()), 20899048083289);
    assert_eq!(part2(input), 273);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5966506063747);
    assert_eq!(part2(input), 1714);
}
