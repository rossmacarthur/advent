use std::collections::{BTreeSet as Set, HashMap as Map};
use std::fmt;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day20.txt");
const BOUND: i64 = 10;

type Vector = vectrs::Vector<i64, 2>;

fn parse_tile(s: &str) -> (i64, Set<Vector>) {
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
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '#' => Some(i),
                    '.' => None,
                    v => panic!("unrecognized image value `{}`", v),
                })
                .map(move |x| Vector::from((x as i64, y as i64)))
        })
        .collect();
    (id, vectors)
}

fn parse_input(s: &str) -> Map<i64, Set<Vector>> {
    s.split("\n\n").map(parse_tile).collect()
}

pub fn default_input() -> Map<i64, Set<Vector>> {
    parse_input(INPUT)
}

fn wrap(v: Vector) -> Vector {
    v.into_iter().map(|n| n.rem_euclid(BOUND)).collect()
}

fn negate(v: Vector) -> Vector {
    v.into_iter().map(|n| -n).collect()
}

fn invert(d: i64) -> i64 {
    (BOUND - 1 - d) % BOUND
}

fn invert_vector(mut v: Vector, dim: usize) -> Vector {
    v[dim] = invert(v[dim]);
    v
}

fn invert_vectors(tile: &Set<Vector>, dim: usize) -> Set<Vector> {
    tile.iter()
        .copied()
        .map(|v| invert_vector(v, dim))
        .collect()
}

fn rotate_vector(v: Vector, angle: i64) -> Vector {
    let x = v.x();
    let y = v.y();
    wrap(match angle {
        0 => Vector::from([x, y]),
        90 => Vector::from([-y, x]),
        180 => Vector::from([-x, -y]),
        270 => Vector::from([y, -x]),
        _ => unimplemented!(),
    })
}

fn rotate_vectors(tile: &Set<Vector>, angle: i64) -> Set<Vector> {
    tile.iter()
        .copied()
        .map(|v| rotate_vector(v, angle))
        .collect()
}

fn tile_edges(tile: &Set<Vector>, direction: Vector) -> Set<Vector> {
    let dimension: usize = direction.into_iter().position(|n| n != 0).unwrap();
    let dimension_values = tile.into_iter().map(|v| v[dimension]);
    let edge_d = match direction[dimension] {
        1 => dimension_values.max().unwrap(),
        -1 => dimension_values.min().unwrap(),
        _ => panic!("invalid direction"),
    };
    tile.into_iter()
        .copied()
        .filter(|v| v[dimension] == edge_d)
        .collect()
}

fn tile_edges_all(tile: &Set<Vector>) -> Set<Set<Vector>> {
    let mut edges = Set::new();
    for direction in &[[1, 0], [-1, 0], [0, 1], [0, -1]] {
        let edge = tile_edges(tile, Vector::from(*direction));
        for dimension in &[0, 1] {
            let inverted = invert_vectors(&edge, *dimension);
            for angle in &[0, 90, 180, 270] {
                edges.insert(rotate_vectors(&inverted, *angle));
            }
        }
    }
    edges
}

fn is_neighbour(tile: &Set<Vector>, for_tile: &Set<Vector>) -> bool {
    tile_edges_all(tile)
        .intersection(&tile_edges_all(for_tile))
        .count() > 0
}

fn valid_neighbours(tiles: &Map<i64, Set<Vector>>, for_id: i64) -> Vec<i64> {
    let for_tile = &tiles[&for_id];
    tiles
        .iter()
        .filter(|(tile_id, _)| **tile_id != for_id)
        .filter(|(tile_id, tile)| is_neighbour(tile, &for_tile))
        .map(|(tile_id, _)| *tile_id)
        .collect()
}

pub fn part1(tiles: &Map<i64, Set<Vector>>) -> usize {

    tiles
        .iter()
        .map(|(tile_id, _)| (*tile_id, valid_neighbours(tiles, *tile_id).len()))
        .sorted_by_key(|(_, count)| *count)
        .inspect(|hello| println!("{:?}", hello))
        .take(1)
        .count();


    todo!()
}

pub fn part2(input: &Map<i64, Set<Vector>>) -> usize {
    todo!()
}

#[test]
fn ex1() {
    let tiles = parse_input(include_str!("input/day20_ex1.txt"));
    assert_eq!(part1(&tiles), 20899048083289);
}

#[test]
fn basic() {
    let tile = parse_tile(
        r#"Tile 0:
###
###
###
"#,
    );
    println!("{:?}", tile_edges_all(&tile.1));
    assert!(false);
}

// 0 0 0 0
// 0 0 0 0
// 0 0 0 0
