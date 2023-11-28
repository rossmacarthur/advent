use advent::prelude::*;

fn parse_range(input: &str) -> [i64; 2] {
    input
        .trim_start_matches("x=")
        .trim_start_matches("y=")
        .split("..")
        .map(str::parse)
        .map(Result::unwrap)
        .next_array()
        .unwrap()
}

fn parse_vein(input: &str) -> impl Iterator<Item = Vector2> {
    let [left, right] = input.split(", ").next_array().unwrap();
    if let Some(s) = left.strip_prefix("x=") {
        let x = s.parse().unwrap();
        let [y1, y2] = parse_range(right);
        Either::Left((y1..=y2).map(move |y| vector![x, y]))
    } else if let Some(s) = left.strip_prefix("y=") {
        let y = s.parse().unwrap();
        let [x1, x2] = parse_range(right);
        Either::Right((x1..=x2).map(move |x| vector![x, y]))
    } else {
        unreachable!()
    }
}

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    input
        .lines()
        .flat_map(parse_vein)
        .map(|p| (p, Tile::Clay))
        .collect()
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_input!(2018 / 17))
}

const LEFT: Vector2 = vector![-1, 0];
const RIGHT: Vector2 = vector![1, 0];
const UP: Vector2 = vector![0, -1];
const DOWN: Vector2 = vector![0, 1];

#[derive(Debug, Clone, Copy)]
enum Tile {
    // Sand, // (represented by None)
    Clay,
    Flowing,
    Settled,
}

// Returns `true` if `p` is a tile that water can flow through.
fn is_permeable(map: &HashMap<Vector2, Tile>, p: Vector2) -> bool {
    matches!(map.get(&p), Some(Tile::Flowing) | None)
}

// Returns `true` if `p` is a tile that water can settle on.
fn is_settleable(map: &HashMap<Vector2, Tile>, p: Vector2) -> bool {
    matches!(map.get(&p), Some(Tile::Clay | Tile::Settled))
}

fn is_clay(map: &HashMap<Vector2, Tile>, p: Vector2) -> bool {
    matches!(map.get(&p), Some(Tile::Clay))
}

fn drain(mut map: HashMap<Vector2, Tile>) -> HashMap<Vector2, Tile> {
    let min_y = map.keys().map(|p| p.y).min().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();

    let spring = vector![500, min_y];
    let mut qy = VecDeque::from([spring]);
    let mut qx = VecDeque::new();

    while !qy.is_empty() || !qx.is_empty() {
        while let Some(p) = qy.pop_front() {
            // Let the water flow down as far as possible.
            let mut n = p;
            while is_permeable(&map, n) && n.y <= max_y {
                map.insert(n, Tile::Flowing);
                n += DOWN;
            }
            // If we go off the edge of the map then stop.
            if n.y > max_y {
                continue;
            }
            // Otherwise we can no longer flow downwards so we must see if the
            // water settles in the position we just came from.
            qx.push_back(n + UP);
        }

        while let Some(p) = qx.pop_front() {
            // Now try to flow left and right as far as possible, we can only
            // flow over clay or settled water.
            let mut l = p;
            while is_permeable(&map, l) && is_settleable(&map, l + DOWN) {
                l += LEFT;
            }
            let mut r = p;
            while is_permeable(&map, r) && is_settleable(&map, r + DOWN) {
                r += RIGHT;
            }

            if is_clay(&map, l) && is_clay(&map, r) {
                // If both the furthest left and furthest right points are clay,
                // then everything between them settles.
                for x in (l.x + 1)..=(r.x - 1) {
                    map.insert(vector![x, l.y], Tile::Settled);
                }
                qx.push_back(p + UP);
            } else {
                // Otherwise water flows between these points and potentially
                // flows downwards.
                for x in (l.x + 1)..=(r.x - 1) {
                    map.insert(vector![x, l.y], Tile::Flowing);
                }
                if is_permeable(&map, l) {
                    qy.push_back(l);
                }
                if is_permeable(&map, r) {
                    qy.push_back(r);
                }
            }
        }
    }

    map
}

fn part1(map: HashMap<Vector2, Tile>) -> usize {
    drain(map)
        .values()
        .filter(|t| matches!(t, Tile::Flowing | Tile::Settled))
        .count()
}

fn part2(map: HashMap<Vector2, Tile>) -> usize {
    drain(map)
        .values()
        .filter(|t| matches!(t, Tile::Settled))
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504",
    );
    assert_eq!(part1(input.clone()), 57);
    assert_eq!(part2(input), 29);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 36790);
    assert_eq!(part2(input), 30765);
}
