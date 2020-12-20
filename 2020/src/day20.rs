use std::collections::{BTreeSet as Set, HashMap as Map, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("input/day20.txt");

type Vector = vectrs::Vector<i64, 2>;
type Point = Vector;
type Edge = Vec<Pixel>;
type Tile = Map<Point, Pixel>;
type Image = Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pixel {
    Black,
    White,
}

fn parse_tile(s: &str) -> (i64, Tile) {
    let mut lines = s.lines();
    let id = lines
        .next()
        .unwrap()
        .trim_start_matches("Tile ")
        .trim_end_matches(":")
        .parse()
        .unwrap();
    let vectors = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| match c {
                    '#' => Pixel::White,
                    '.' => Pixel::Black,
                    p => panic!("unrecognized pixel value `{}`", p),
                })
                .enumerate()
                .map(move |(x, pixel)| (Point::from((x as i64, y as i64)), pixel))
        })
        .collect();
    (id, vectors)
}

fn parse_input(s: &str) -> Map<i64, Tile> {
    s.split("\n\n").map(parse_tile).collect()
}

pub fn default_input() -> Map<i64, Tile> {
    parse_input(INPUT)
}

fn tile_edge(tile: &Tile, direction: Vector) -> Edge {
    let d: usize = direction.into_iter().position(|n| n != 0).unwrap();
    let values = tile.into_iter().map(|(point, _)| point[d]);
    let edge = match direction[d] {
        1 => values.max().unwrap(),
        -1 => values.min().unwrap(),
        _ => panic!("invalid direction"),
    };
    tile.iter()
        .filter(|(point, _)| point[d] == edge)
        .sorted_by_key(|(point, _)| *point)
        .map(|(_, pixel)| *pixel)
        .collect()
}

fn tile_edges(tile: &Tile) -> Set<Edge> {
    let mut edges = Set::new();
    for direction in &[[1, 0], [-1, 0], [0, 1], [0, -1]] {
        let direction = Vector::from(*direction);
        let edge = tile_edge(tile, direction);
        edges.insert(edge.clone());
        edges.insert(edge.into_iter().rev().collect());
    }
    edges
}

fn adjacent_tiles(tiles: &Map<i64, Tile>) -> Vec<(i64, Set<i64>)> {
    let edges: Map<_, _> = tiles
        .iter()
        .map(|(tile_id, tile)| (tile_id, tile_edges(tile)))
        .collect();
    let mut adjacents = Map::new();
    for tile_id_a in edges.keys().copied() {
        for tile_id_b in edges.keys().copied().filter(|id| *id != tile_id_a) {
            if edges[tile_id_a].intersection(&edges[tile_id_b]).count() > 0 {
                adjacents
                    .entry(*tile_id_a)
                    .or_insert_with(Set::new)
                    .insert(*tile_id_b);
            }
        }
    }
    adjacents
        .into_iter()
        .sorted_by_key(|(_, adjacents)| adjacents.len())
        .collect()
}

fn rotate_point(point: Point, angle: i64, bound: i64) -> Point {
    let x = point.x();
    let y = point.y();
    let point = match angle {
        0 => Point::from([x, y]),
        90 => Point::from([-y, x]),
        180 => Point::from([-x, -y]),
        270 => Point::from([y, -x]),
        _ => unimplemented!(),
    };
    point.into_iter().map(|n| n.rem_euclid(bound)).collect()
}

fn rotate_points(points: &[Point], angle: i64, bound: i64) -> Vec<Vector> {
    points
        .iter()
        .copied()
        .map(|v| rotate_point(v, angle, bound))
        .collect()
}

fn corners(bound: i64) -> Vec<(Point, Point)> {
    let points = [Point::from((1, 0)), Point::from((0, 1))];
    [0, 90, 180, 270]
        .iter()
        .map(|angle| {
            rotate_points(&points, *angle, bound)
                .into_iter()
                .next_tuple()
                .unwrap()
        })
        .collect()
}

//
// # #
// #


fn place_tile(adjacents: &Map<i64, Set<i64>>, tile_id: i64, layout: &mut Map<Point, i64>) {

    if layout.len() == 0 {
        // this is the first tile, just put it in the top left hand corner
        layout.insert(Point::from([0, 0]), tile_id);
        for adjacent_id in &adjacents[&tile_id] {
            place_tile(adjacents, *adjacent_id, layout);
        }
    } else {
        todo!()

    }

}

fn pick_and_place(adjacents: Vec<(i64, Set<i64>)>) -> Map<Point, i64> {
    let bound = (adjacents.len() as f64).sqrt() as i64;
    let first_tile_id = adjacents.iter().map(|(tile_id, _)| *tile_id).next().unwrap();
    let adjacents: Map<i64, Set<i64>> = adjacents.into_iter().collect();
    let mut layout = Map::new();
    place_tile(&adjacents, first_tile_id, &mut layout);
    layout
}

pub fn part1(tiles: &Map<i64, Tile>) -> i64 {
    adjacent_tiles(tiles)
        .into_iter()
        .take(4)
        .map(|(tile_id, _)| tile_id)
        .product()
}

pub fn part2(tiles: &Map<i64, Tile>) -> usize {
    pick_and_place(adjacent_tiles(tiles));
    0
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 5966506063747);
}
