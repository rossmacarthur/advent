use advent::prelude::*;

fn parse_input(input: &str) -> (Vector2, Vector2, HashMap<Vector2, Tile>) {
    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '.' => Tile::Clear,
        '#' => Tile::Wall,
        '>' => Tile::Blizzard(vector![1, 0]),
        '<' => Tile::Blizzard(vector![-1, 0]),
        '^' => Tile::Blizzard(vector![0, -1]),
        'v' => Tile::Blizzard(vector![0, 1]),
        _ => panic!("unexpected character `{c}`"),
    });

    // Find the starting and ending points
    let (min_y, max_y) = map.keys().map(|p| p.y).minmax().into_option().unwrap();
    let mut start = vector![-1, -1];
    let mut end = vector![-1, -1];
    for (&p, t) in &map {
        if matches!(t, Tile::Clear) {
            if p.y == min_y {
                start = p;
            }
            if p.y == max_y {
                end = p;
            }
        }
    }

    (start, end, map)
}

fn default_input() -> (Vector2, Vector2, HashMap<Vector2, Tile>) {
    parse_input(include_str!("input/24.txt"))
}

#[derive(Clone, Copy)]
enum Tile {
    Clear,
    Wall,
    Blizzard(Vector2),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TileMulti {
    Wall,
    Blizzards(Vec<Vector2>),
}

/// Returns the next blizzard state for the given blizzard state.
fn next_state(map: &HashMap<Vector2, TileMulti>, bounds: [i64; 4]) -> HashMap<Vector2, TileMulti> {
    let [min_x, max_x, min_y, max_y] = bounds;
    let mut next: HashMap<Vector2, TileMulti> = HashMap::new();
    for (&p, t) in map {
        match t {
            TileMulti::Wall => {
                next.insert(p, TileMulti::Wall);
            }
            TileMulti::Blizzards(bs) => {
                for &b in bs {
                    let mut n = p + b;
                    if n.x == min_x {
                        n.x = max_x - 1;
                    } else if n.x == max_x {
                        n.x = min_x + 1;
                    }
                    if n.y == min_y {
                        n.y = max_y - 1;
                    } else if n.y == max_y {
                        n.y = min_y + 1;
                    }
                    match next
                        .entry(n)
                        .or_insert_with(|| TileMulti::Blizzards(Vec::new()))
                    {
                        TileMulti::Blizzards(bs) => bs.push(b),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    next
}

/// Generates all clear points for every tick.
fn clear_maps(map: HashMap<Vector2, Tile>) -> Vec<HashSet<Vector2>> {
    let (min_x, max_x) = map.keys().map(|p| p.x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|p| p.y).minmax().into_option().unwrap();
    let bounds = [min_x, max_x, min_y, max_y];

    let mut maps: Vec<HashSet<Vector2>> = Vec::new();

    let start: HashMap<_, _> = map
        .into_iter()
        .filter_map(|(p, t)| {
            let t = match t {
                Tile::Clear => return None,
                Tile::Wall => TileMulti::Wall,
                Tile::Blizzard(bz) => TileMulti::Blizzards(vec![bz]),
            };
            Some((p, t))
        })
        .collect();

    let mut blizzard = start.clone();
    loop {
        // Convert the map of blizzards to a set of clear points instead
        let clear = iproduct!(min_x..=max_x, min_y..=max_y)
            .filter_map(|(x, y)| {
                let p = vector![x, y];
                (!blizzard.contains_key(&p)).some(p)
            })
            .collect();
        maps.push(clear);
        let next = next_state(&blizzard, bounds);
        if next == start {
            break;
        }
        blizzard = next;
    }
    maps
}

/// Finds the shortest path across the map
fn shortest(maps: &[HashSet<Vector2>], start: Vector2, end: Vector2, tick: usize) -> usize {
    let mut q = VecDeque::from([(start, tick)]);
    let mut visited = HashSet::new();
    while let Some((pos, tick)) = q.pop_front() {
        if !visited.insert((pos, tick)) {
            continue;
        }
        if pos == end {
            return tick;
        }
        let available = &maps[(tick + 1) % maps.len()];
        for d in vectors!([0, -1], [-1, 0], [0, 1], [1, 0]) {
            let next = pos + d;
            if available.contains(&next) {
                q.push_back((next, tick + 1));
            }
        }
        if available.contains(&pos) {
            q.push_back((pos, tick + 1));
        }
    }
    panic!("no path found")
}

fn part1((start, end, map): (Vector2, Vector2, HashMap<Vector2, Tile>)) -> usize {
    let maps = clear_maps(map);
    shortest(&maps, start, end, 0)
}

fn part2((start, end, map): (Vector2, Vector2, HashMap<Vector2, Tile>)) -> usize {
    let maps = clear_maps(map);
    let t1 = shortest(&maps, start, end, 0);
    let t2 = shortest(&maps, end, start, t1);
    shortest(&maps, start, end, t2)
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
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
    );
    assert_eq!(part1(input.clone()), 18);
    assert_eq!(part2(input), 54);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 279);
    assert_eq!(part2(input), 762);
}
