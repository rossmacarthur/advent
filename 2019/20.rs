use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

use vectrix::{parse_map, Vector2, CARDINALS};

type Vector = Vector2<i64>;

fn parse_input(input: &str) -> HashMap<Vector, Tile> {
    enum T {
        Space,
        Wall,
        Floor,
        Label(char),
    }

    let map: HashMap<_, _> = parse_map(input, |c| match c {
        ' ' => T::Space,
        '#' => T::Wall,
        '.' => T::Floor,
        c @ 'A'..='Z' => T::Label(c),
        c => panic!("unexpected character `{}`", c),
    });

    // Find the bounds of the outer rectangle of wall tiles, so that we can tell
    // if a portal is an outer or inner one later.
    let (x_min, x_max, y_min, y_max) = map
        .iter()
        .filter(|(_, t)| matches!(t, T::Wall))
        .map(|(v, _)| v)
        .fold(
            (i64::MAX, 0, i64::MAX, 0),
            |(x_min, x_max, y_min, y_max), v| {
                (
                    min(v.x, x_min),
                    max(v.x, x_max),
                    min(v.y, y_min),
                    max(v.y, y_max),
                )
            },
        );

    // Find the labeled floor tiles by searching for floor tiles that have two
    // labels in a single adjacent direction.
    map.iter()
        .filter(|(_, t)| matches!(t, T::Floor))
        .map(|(v, _)| {
            let portal = CARDINALS.iter().find_map(|d| match map.get(&(v + d)) {
                Some(&T::Label(a)) => match map.get(&(v + d * 2)) {
                    Some(&T::Label(b)) => {
                        let t = match (a, b) {
                            ('A', 'A') => Tile::Start,
                            ('Z', 'Z') => Tile::End,
                            (a, b) => {
                                match v.x == x_min || v.x == x_max || v.y == y_min || v.y == y_max {
                                    true => Tile::OutPortal(id(a, b)),
                                    false => Tile::InPortal(id(a, b)),
                                }
                            }
                        };
                        Some((*v, t))
                    }
                    _ => None,
                },
                _ => None,
            });
            match portal {
                Some(tile) => tile,
                None => (*v, Tile::Floor),
            }
        })
        .collect()
}

fn default_input() -> HashMap<Vector, Tile> {
    parse_input(include_str!("input/20.txt"))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Start,
    End,
    InPortal(u32),
    OutPortal(u32),
}

impl Tile {
    fn complements(&self, other: &Tile) -> bool {
        match (self, other) {
            (Tile::InPortal(i), Tile::OutPortal(o)) => i == o,
            (Tile::OutPortal(o), Tile::InPortal(i)) => o == i,
            _ => false,
        }
    }
}

/// Represent two chars as a unique identifier.
///
/// The order of the chars doesn't matter, the same id will always be generated.
fn id(a: char, b: char) -> u32 {
    let id = |a, b| 26 * (a as u32 - 'A' as u32) + (b as u32 - 'A' as u32);
    match a < b {
        true => id(a, b),
        false => id(b, a),
    }
}

/// Find the shortest path from the start tile to the end tile.
///
/// We do this by using a priority queue of states each representing a position
/// on a path from the start to the end. Each state contains the following:
/// - The total distance from the start.
/// - The current position along the path from the start.
/// - The recursion level.
///
/// Each iteration we pick the state that has the shortest distance and we push
/// all valid navigation directions from that tile onto the queue until we can
/// navigate to the end tile.
fn shortest(map: &HashMap<Vector, Tile>, recurse: bool) -> usize {
    // Find the location of the starting tile.
    let start = map
        .iter()
        .find_map(|(v, t)| matches!(t, Tile::Start).then(|| *v))
        .unwrap();

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), start, 0));

    while let Some((Reverse(dist), pos, lvl)) = pq.pop() {
        if !visited.insert((pos, lvl)) {
            continue;
        }

        // Pushes all valid surrounding cardinal points onto the queue.
        let cardinals = || {
            CARDINALS.iter().filter_map(|d| {
                let next = pos + d;
                map.contains_key(&next)
                    .then(|| (Reverse(dist + 1), next, lvl))
            })
        };

        // Pushes the matching portal onto the queue.
        let matching = |portal: Tile, lvl: usize| {
            let next = map
                .iter()
                .find_map(|(v, t)| (*v != pos && portal.complements(t)).then(|| *v))
                .unwrap();
            (Reverse(dist + 1), next, lvl)
        };

        match map[&pos] {
            // For inner portals we navigate to the valid surrounding points and
            // the matching portal. Navigating to the matching outer portal
            // increases the recursion level.
            portal @ Tile::InPortal(_) => {
                pq.push(matching(portal, lvl + 1));
                pq.extend(cardinals());
            }

            // For outer portals we navigate to the valid surrounding points and
            // the matching portal. Navigating to the matching inner portal
            // descreases the recursion level. Note: it is not valid to navigate
            // to outer portals on the outermost level.
            portal @ Tile::OutPortal(_) if !recurse || lvl != 0 => {
                pq.push(matching(portal, lvl.saturating_sub(1)));
                pq.extend(cardinals());
            }

            // In the basic case navigate to the valid surrounding points.
            Tile::Floor | Tile::Start => {
                pq.extend(cardinals());
            }

            // If recursion is enabled the end tile is only valid on the
            // outermost level.
            Tile::End if !recurse || lvl == 0 => {
                return dist;
            }

            _ => {}
        }
    }
    panic!("no path found")
}

fn part1(map: &HashMap<Vector, Tile>) -> usize {
    shortest(map, false)
}

fn part2(map: &HashMap<Vector, Tile>) -> usize {
    shortest(map, true)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z",
    );
    assert_eq!(part1(&input), 23);
    assert_eq!(part2(&input), 26);
}

#[test]
fn example2() {
    let input = parse_input(
        "                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P",
    );
    assert_eq!(part1(&input), 58);
}

#[test]
fn example3() {
    let input = parse_input(
        "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M",
    );
    assert_eq!(part2(&input), 396);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 490);
    assert_eq!(part2(&input), 5648);
}
