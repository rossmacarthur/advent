use std::ops::BitOr;

use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    parse_map(input, |c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Floor,
        '@' => Tile::Entrance(bit('0')),
        'a'..='z' => Tile::Key(bit(c)),
        'A'..='Z' => Tile::Door(bit(c.to_ascii_lowercase())),
        c => panic!("unexpected character `{c}`"),
    })
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_input!(2019 / 18))
}

const CENTER: Vector2 = vector![0, 0];
const NORTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];
const SOUTH: Vector2 = vector![0, -1];
const WEST: Vector2 = vector![-1, 0];
const NORTH_EAST: Vector2 = vector![1, 1];
const SOUTH_WEST: Vector2 = vector![-1, -1];
const NORTH_WEST: Vector2 = vector![1, -1];
const SOUTH_EAST: Vector2 = vector![-1, 1];
const CARDINALS: [Vector2; 4] = [NORTH, EAST, SOUTH, WEST];

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Floor,
    Entrance(u32),
    Key(u32),
    Door(u32),
}

impl Tile {
    fn is_entrance(&self) -> bool {
        matches!(*self, Tile::Entrance(_))
    }

    fn is_key(&self) -> bool {
        matches!(*self, Tile::Key(_))
    }

    fn is_interesting(&self) -> bool {
        matches!(*self, Tile::Entrance(_) | Tile::Key(_))
    }

    fn value(&self) -> u32 {
        match *self {
            Tile::Entrance(k) | Tile::Key(k) | Tile::Door(k) => k,
            t => panic!("no value for tile `{t:?}`"),
        }
    }
}

/// Represent the characters as bits.
///
/// This makes the solution much faster and memory efficient because it allows
/// us to store multiple sources and keys in a single integer, which allows us
/// to perform bitwise operations on the data.
fn bit(c: char) -> u32 {
    match c {
        'a'..='z' => 1 << (c as u8 - b'a'),
        '0'..='3' => 1 << (c as u8 - b'0' + 26),
        c => panic!("unsupported character `{c}`"),
    }
}

/// Iterate over the set bits in a bitmap.
fn iter(bitmap: u32) -> impl Iterator<Item = u32> + 'static {
    iter::successors(Some(1u32), |n| n.checked_mul(2))
        .take(32)
        .filter(move |b| bitmap & b != 0)
}

/// Calculate the distances to all the keys from the given source.
///
/// This function uses a simple breadth-first search to navigate to every key
/// from the source. The doors that are passed through are recorded as well. The
/// result is a map of destination key to distance and doors passed through.
fn distances(map: &HashMap<Vector2, Tile>, source: Vector2) -> HashMap<u32, (usize, u32)> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();
    visited.insert(source);
    frontier.push_back((source, 0, 0));
    while let Some((pos, dist, doors)) = frontier.pop_front() {
        for d in CARDINALS {
            let next = pos + d;
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            match &map[&next] {
                Tile::Wall => {
                    continue;
                }
                Tile::Key(k) => {
                    distances.insert(*k, (dist + 1, doors));
                    frontier.push_back((next, dist + 1, doors));
                }
                Tile::Door(k) => {
                    let doors = doors | k;
                    frontier.push_back((next, dist + 1, doors));
                }
                Tile::Entrance(_) | Tile::Floor => {
                    frontier.push_back((next, dist + 1, doors));
                }
            }
        }
    }
    distances
}

/// Find the shortest path from entrance(s) to pick up all keys.
///
/// We start by building a distance graph for all entrances and keys. This graph
/// ignores doors for the distance but records which ones need to be passed
/// through.
///
/// Then we use Dijkstra's algorithm to find the shortest path. We do this by
/// building a priority queue of states. A state contains the following:
/// - The sum of the distances from each entrance.
/// - The current node for each path from each entrance.
/// - The keys that we have collected along the way.
///
/// Each iteration we pick the state that has the shortest distance and we try
/// and navigate each source to all the next possible nodes in the graph until
/// we have found all the keys.
fn shortest(map: &HashMap<Vector2, Tile>) -> usize {
    let graph: HashMap<_, _> = map
        .iter()
        .filter(|(_, t)| t.is_interesting())
        .map(|(p, t)| (t.value(), distances(map, *p)))
        .collect();

    let all_keys = map
        .values()
        .filter(|t| t.is_key())
        .map(|t| t.value())
        .fold(0, BitOr::bitor);

    let sources = map
        .values()
        .filter(|t| t.is_entrance())
        .map(|t| t.value())
        .fold(0, BitOr::bitor);

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), sources, 0));
    while let Some((Reverse(dist), sources, keys)) = pq.pop() {
        if visited.contains(&(sources, keys)) {
            continue;
        }
        visited.insert((sources, keys));
        if keys == all_keys {
            return dist;
        }
        for source in iter(sources) {
            for (next, &(d, doors)) in &graph[&source] {
                if doors & keys != doors {
                    continue;
                }
                let sources = (sources ^ source) | next;
                pq.push((Reverse(dist + d), sources, keys | next));
            }
        }
    }
    panic!("no path found")
}

fn part1(map: HashMap<Vector2, Tile>) -> usize {
    shortest(&map)
}

fn part2(mut map: HashMap<Vector2, Tile>) -> usize {
    // Update the map
    let to_update = &[
        (CENTER, Tile::Wall),
        (NORTH, Tile::Wall),
        (EAST, Tile::Wall),
        (SOUTH, Tile::Wall),
        (WEST, Tile::Wall),
        (NORTH_EAST, Tile::Entrance(bit('0'))),
        (SOUTH_EAST, Tile::Entrance(bit('1'))),
        (SOUTH_WEST, Tile::Entrance(bit('2'))),
        (NORTH_WEST, Tile::Entrance(bit('3'))),
    ];
    let entrance = map
        .iter()
        .find_map(|(p, t)| t.is_entrance().some(*p))
        .unwrap();
    for &(d, t) in to_update {
        map.insert(entrance + d, t);
    }

    shortest(&map)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
#########
#b.A.@.a#
#########",
    );
    assert_eq!(part1(input), 8);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
    );
    assert_eq!(part1(input), 86);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
    );
    assert_eq!(part1(input), 132);
}

#[test]
fn example4() {
    let input = parse_input(
        "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
    );
    assert_eq!(part1(input), 136);
}

#[test]
fn example5() {
    let input = parse_input(
        "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
    );
    assert_eq!(part1(input), 81);
}

#[test]
fn example6() {
    let input = parse_input(
        "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
    );
    assert_eq!(part1(input.clone()), 26);
    assert_eq!(part2(input), 8);
}

#[test]
fn example7() {
    let input = parse_input(
        "\
###############
#d.ABC.#.....a#
######...######
#######@#######
######...######
#b.....#.....c#
###############",
    );
    assert_eq!(part1(input.clone()), 52);
    assert_eq!(part2(input), 24);
}

#[test]
fn example8() {
    let input = parse_input(
        "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
######@######
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
    );
    assert_eq!(part1(input.clone()), 118);
    assert_eq!(part2(input), 72);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3646);
    assert_eq!(part2(input), 1730);
}
