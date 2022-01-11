use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
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
    let [min_x, max_x, min_y, max_y] = map
        .iter()
        .filter(|(_, t)| matches!(t, T::Wall))
        .map(|(v, _)| v)
        .fold([i64::MAX, i64::MIN, i64::MAX, i64::MIN], |acc, p| {
            let [min_x, max_x, min_y, max_y] = acc;
            [
                min(min_x, p.x),
                max(max_x, p.x),
                min(min_y, p.y),
                max(max_y, p.y),
            ]
        });

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
                                match v.x == min_x || v.x == max_x || v.y == min_y || v.y == max_y {
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

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_str!("input/20.txt"))
}

const NORTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];
const SOUTH: Vector2 = vector![0, -1];
const WEST: Vector2 = vector![-1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, EAST, SOUTH, WEST];

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
fn shortest(map: HashMap<Vector2, Tile>, recurse: bool) -> usize {
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
            // decreases the recursion level. Note: it is not valid to navigate
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

fn part1(map: HashMap<Vector2, Tile>) -> usize {
    shortest(map, false)
}

fn part2(map: HashMap<Vector2, Tile>) -> usize {
    shortest(map, true)
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
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
    assert_eq!(part1(input.clone()), 23);
    assert_eq!(part2(input), 26);
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
    assert_eq!(part1(input), 58);
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
    assert_eq!(part2(input), 396);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 490);
    assert_eq!(part2(input), 5648);
}
