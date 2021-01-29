use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex_macro::regex;
use vector::i64::xy::vector;

use crate::map::parse_map_set;

const INPUT: &str = include_str!("input/day20.txt");

type Edge = Vec<Pixel>;
type Image = Vec<Vec<Pixel>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pixel {
    Black,
    White,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Tile {
    id: i64,
    pixels: Vec<Vec<Pixel>>,
}

fn parse_tile(input: &str) -> Tile {
    let captures = regex!(r"(?s)Tile (\d+):\n(.*)").captures(input).unwrap();
    Tile::parse(captures[1].parse().unwrap(), &captures[2])
}

fn parse_input(s: &str) -> Vec<Tile> {
    s.split("\n\n").map(parse_tile).collect()
}

pub fn default_input() -> Vec<Tile> {
    parse_input(INPUT)
}

fn reversed(mut edge: Edge) -> Edge {
    edge.reverse();
    edge
}

impl Tile {
    fn parse(id: i64, input: &str) -> Self {
        let pixels: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Pixel::Black,
                        '#' => Pixel::White,
                        p => panic!("unrecognized pixel value `{}`", p),
                    })
                    .collect()
            })
            .collect();
        Self { id, pixels }
    }

    fn rows(&self) -> usize {
        self.pixels.len()
    }

    fn cols(&self) -> usize {
        self.pixels[0].len()
    }

    fn top(&self) -> Edge {
        self.pixels[0].clone()
    }

    fn bottom(&self) -> Edge {
        self.pixels[self.rows() - 1].clone()
    }

    fn left(&self) -> Edge {
        self.pixels.iter().map(|row| row[0]).collect()
    }

    fn right(&self) -> Edge {
        self.pixels.iter().map(|row| row[self.cols() - 1]).collect()
    }

    fn edges(&self) -> Vec<Edge> {
        vec![self.top(), self.right(), self.bottom(), self.left()]
    }

    fn rotate(&mut self) {
        let rows = self.cols();
        let cols = self.rows();
        let mut pixels = vec![vec![Pixel::Black; cols]; rows];
        for row in 0..rows {
            for col in 0..cols {
                pixels[row][col] = self.pixels[self.rows() - col - 1][row];
            }
        }
        self.pixels = pixels;
    }

    fn flip_v(&mut self) {
        let rows = self.rows();
        for i in 0..(rows / 2) {
            self.pixels.swap(i, rows - 1 - i);
        }
    }

    fn flip_h(&mut self) {
        for row in &mut self.pixels {
            row.reverse();
        }
    }
}

fn edge_matches(tiles: &[Tile]) -> HashMap<Edge, HashSet<i64>> {
    let mut matches = HashMap::new();
    for tile in tiles {
        for edge in tile.edges() {
            matches
                .entry(edge.clone())
                .or_insert_with(HashSet::new)
                .insert(tile.id);
            matches
                .entry(reversed(edge))
                .or_insert_with(HashSet::new)
                .insert(tile.id);
        }
    }
    matches
}

fn find_corners(matches: &HashMap<Edge, HashSet<i64>>) -> Vec<i64> {
    // Count all the border edges, a border edge is an edge that doesn't match
    // any other edge. We can use this to figure out the corner tile IDs because
    // they will have the most border edges.
    matches
        .values()
        .filter(|ids| ids.len() == 1)
        .map(|ids| ids.into_iter().next().unwrap())
        .fold(HashMap::new(), |mut borders, tile_id| {
            *borders.entry(*tile_id).or_insert(0) += 1;
            borders
        })
        .into_iter()
        .sorted_by_key(|(_, count)| Reverse(*count))
        .take(4)
        .map(|(id, _)| id)
        .collect()
}

fn is_not_border(matches: &HashMap<Edge, HashSet<i64>>, edge: Edge) -> bool {
    matches[&edge].len() + matches[&reversed(edge.clone())].len() > 2
}

fn is_match(a: Edge, b: Edge) -> bool {
    a == b || a == reversed(b)
}

fn find_left_match(
    tiles: &[Tile],
    matches: &HashMap<Edge, HashSet<i64>>,
    prev_tile: &Tile,
) -> Tile {
    let edge = prev_tile.right();
    let id = *matches[&edge]
        .iter()
        .find(|&id| *id != prev_tile.id)
        .unwrap();
    let mut tile: Tile = tiles.iter().find(|tile| tile.id == id).unwrap().clone();
    while !is_match(edge.clone(), tile.left()) {
        tile.rotate();
    }
    if tile.left() != edge {
        tile.flip_v();
    }
    tile
}

fn find_top_match(tiles: &[Tile], matches: &HashMap<Edge, HashSet<i64>>, prev_tile: &Tile) -> Tile {
    let edge = prev_tile.bottom();
    let id = *matches[&edge]
        .iter()
        .find(|&id| *id != prev_tile.id)
        .unwrap();
    let mut tile: Tile = tiles.iter().find(|tile| tile.id == id).unwrap().clone();
    while !is_match(edge.clone(), tile.top()) {
        tile.rotate();
    }
    if tile.top() != edge {
        tile.flip_h();
    }
    tile
}

fn pick_and_place(tiles: &[Tile]) -> Image {
    let matches = edge_matches(tiles);

    // Start with a corner tile and rotate it until the border edges are on the
    // left and top.
    let corner_tile_id = find_corners(&matches)[0];
    let mut corner_tile = tiles
        .iter()
        .find(|tile| tile.id == corner_tile_id)
        .unwrap()
        .clone();
    while is_not_border(&matches, corner_tile.top()) || is_not_border(&matches, corner_tile.left())
    {
        corner_tile.rotate();
    }

    let dim = (tiles.len() as f64).sqrt() as usize;
    let mut arranged = vec![vec![Tile::default(); dim]; dim];

    // Place the corner tile.
    arranged[0][0] = corner_tile;

    // Place the rest of the first row, by comparing it to the tile on the left.
    for col in 1..dim {
        arranged[0][col] = find_left_match(tiles, &matches, &arranged[0][col - 1]);
    }

    // Place the rest of the rows, by comparing it to the row above.
    for row in 1..dim {
        for col in 0..dim {
            arranged[row][col] = find_top_match(tiles, &matches, &arranged[row - 1][col]);
        }
    }

    // Finally, construct the image
    let cols = arranged[0][0].cols();
    let rows = arranged[0][0].rows();
    let mut image = Vec::new();

    for tiles in arranged {
        for row in 0..rows {
            let mut pixels = Vec::new();
            for tile in &tiles {
                for col in 0..cols {
                    pixels.push(tile.pixels[row][col]);
                }
            }
            image.push(pixels);
        }
    }

    image
}

pub fn part1(tiles: &[Tile]) -> i64 {
    find_corners(&edge_matches(tiles)).into_iter().product()
}

pub fn part2(tiles: &[Tile]) -> usize {
    let mut image = pick_and_place(tiles);

    let rows = image.len() as i64;
    let cols = image[0].len() as i64;



    let monster = parse_map_set("                  #\n#    ##    ##    ###\n #  #  #  #  #  #");

    // loop through the entire image
    loop {
        println!("here");

        let image_set: HashSet<_> = image
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        Pixel::White => Some(x),
                        Pixel::Black => None,
                    })
                    .map(move |x| vector![x as i64, y as i64])
            })
            .collect();

        let mut count = 0;
        for x in 0..cols {
            for y in 0..rows {
                let to_check: HashSet<_> = monster.iter().map(|v| vector![x, y] + v).collect();
                if to_check.is_subset(&image_set) {
                    println!("{:?}", image_set);
                    count += 1;
                }
            }
        }

        if count > 0 {
            return count;
        } else {
            let mut tile = Tile { id: 0, pixels: image };
            tile.rotate();
            image = tile.pixels;
        }



    }
}

#[test]
fn example() {
    let tile = Tile::parse(123, "###\n##.");
    assert!(false);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 5966506063747);
    assert_eq!(part2(&input), 1714);
}
