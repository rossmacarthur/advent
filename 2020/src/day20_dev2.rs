use std::collections::{HashMap, HashSet};
use std::fmt;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day20.txt");
const BOUND: i64 = 10;

type Vector = vectrs::Vector<i64, 2>;

fn parse_tile(s: &str) -> (i64, HashSet<Vector>) {
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

fn parse_input(s: &str) -> HashMap<i64, HashSet<Vector>> {
    s.split("\n\n").map(parse_tile).collect()
}

pub fn default_input() -> HashMap<i64, HashSet<Vector>> {
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

fn invert_vector(v: Vector, dim: usize) -> Vector {
    v[dim] = invert(v[dim]);
    v
}

fn invert_tile(tile: &HashSet<Vector>, dim: usize) -> HashSet<Vector> {
    tile.iter().copied().map(|v| invert_vector(v, dim)).collect()
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

fn rotate_tile(tile: &HashSet<Vector>, angle: i64) -> HashSet<Vector> {
    tile.iter()
        .copied()
        .map(|v| rotate_vector(v, angle))
        .collect()
}

fn edge_vectors(tile: &HashSet<Vector>, direction: Vector) -> HashSet<Vector> {
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

fn transformations(tile_id: i64, tile: &HashSet<Vector>) -> HashMap<Uid, HashSet<Vector>> {
    let mut transformations = HashMap::new();

    for flip in [0, 1].iter().copied() {
        for angle in [0, 90, 180, 270].iter().copied() {
            let rotated = rotate_tile(&tile, angle);

        }
    }

        let rotated = rotate_tile(&tile, angle);
        transformations.insert(
            Uid {
                tile_id,
                angle,
                inversion: Inversion::X,
            },
            invert_tile(&rotated, 0),
        );
        transformations.insert(
            Uid {
                tile_id,
                angle,
                inversion: Inversion::Y,
            },
            invert_tile_y(&rotated),
        );
    }
    transformations
}







#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Uid {
    tile_id: i64,
    angle: i64,
    invert: i64,
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uid({}:{}:{:?})",
            self.tile_id, self.angle, self.inversion,
        )
    }
}


fn pick_and_place(tile_ids: HashSet<i64>, valid: &HashMap<Uid, HashSet<(Vector, Uid)>>) {
    // for tile_ids in tile_ids.into_iter().permutations(tile_ids.len()) {
    //     let image: HashMap<_, _> = (0..12)
    //         .zip(0..12)
    //         .map(Vector::from)
    //         .map(|v| (v, tile_id))
    //         .collect();
    // }
}

pub fn part1(tiles: &HashMap<i64, HashSet<Vector>>) -> usize {
    let directions: Vec<_> = [[1, 0], [-1, 0], [0, 1], [0, -1]]
        .iter()
        .copied()
        .map(Vector::from)
        .collect();

    let tile_ids: Vec<_> = tiles.keys().copied().collect();

    // Get the edges of all the transformations for each tile in each direction.
    // Includes a UID of the transformation.
    // i.e. { tile_id: { direction: { (uid, Vectors), .. } } }
    let edges: HashMap<i64, _> = tiles
        .into_iter()
        .map(|(tile_id, base_tile)| {
            let mut edges = HashMap::new();
            for direction in directions.iter().copied() {
                for (uid, tile) in &transformations(*tile_id, base_tile) {
                    edges
                        .entry(direction)
                        .or_insert_with(Vec::new)
                        .push((*uid, edge_vectors(tile, direction)));
                }
            }
            (*tile_id, edges)
        })
        .collect();

    // Build a map of tile UID and what it can be next to.
    // i.e. { uid: { (direction, uid) } }
    let mut tiles = HashMap::new();
    for (id_a, id_b) in tile_ids.into_iter().tuple_combinations() {
        for direction in directions.iter().copied() {
            let direction_rev = negate_vector(direction);
            for (uid_a, edge_a) in &edges[&id_a][&direction] {
                for (uid_b, edge_b) in &edges[&id_b][&direction_rev] {
                    if edge_b
                        == &edge_a
                            .iter()
                            .map(|v| wrap(v + &direction))
                            .collect::<HashSet<_>>()
                    {
                        tiles
                            .entry(id_a)
                            .or_insert_with(HashMap::new)
                            .entry(uid_a)
                            .or_insert_with(HashMap::new)
                            .entry(direction)
                            .or_insert_with(Vec::new)
                            .push(uid_b);
                        tiles
                            .entry(id_b)
                            .or_insert_with(HashMap::new)
                            .entry(uid_b)
                            .or_insert_with(HashMap::new)
                            .entry(direction_rev)
                            .or_insert_with(Vec::new)
                            .push(uid_a);
                    }
                }
            }
        }
    }

    let result: Vec<_> = tiles
        .into_iter()
        .map(|(tile_id, uids)| {
            (
                tile_id,
                uids.into_iter()
                    .flat_map(|(uid, directions)| {
                        directions
                            .values()
                            .into_iter()
                            .flat_map(|uids| uids.into_iter().map(|uid| uid.tile_id))
                            .collect::<HashSet<i64>>()
                    })
                    .collect::<HashSet<i64>>(),
            )
        })
        .filter(|(tile_id, neighbours)| neighbours.len() == 2)
        .collect();

    println!("{:?}", result);

    todo!()
}

pub fn part2(input: &HashMap<i64, HashSet<Vector>>) -> usize {
    todo!()
}

#[test]
fn ex0() {
    let tiles = parse_input(
        r#"Tile 0:
.#
#.

Tile 1:
.#
#.

Tile 2:
#.
.#

Tile 3:
#.
.#
"#,
    );

    part1(&tiles);
}

#[test]
fn ex1() {
    let tiles = parse_input(include_str!("input/day20_ex1.txt"));
    assert_eq!(part1(&tiles), 20899048083289);
}

// 0 0 0 0
// 0 0 0 0
// 0 0 0 0
