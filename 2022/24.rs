use advent::prelude::*;

fn parse_input(input: &str) -> (Vector2, Vector2, HashMap<Vector2, Tile>) {
    #[derive(Clone, Copy)]
    enum Raw {
        Clear,
        Wall,
        Blizzard(Vector2),
    }

    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '.' => Raw::Clear,
        '#' => Raw::Wall,
        '>' => Raw::Blizzard(vector![1, 0]),
        '<' => Raw::Blizzard(vector![-1, 0]),
        '^' => Raw::Blizzard(vector![0, -1]),
        'v' => Raw::Blizzard(vector![0, 1]),
        _ => panic!("unexpected character `{c}`"),
    });

    // Find the starting and ending points
    let (min_y, max_y) = map.keys().map(|p| p.y).minmax().into_option().unwrap();
    let mut start = vector![-1, -1];
    let mut end = vector![-1, -1];
    for (&p, t) in &map {
        if matches!(t, Raw::Clear) {
            if p.y == min_y {
                start = p;
            }
            if p.y == max_y {
                end = p;
            }
        }
    }

    // Convert map so that it can handle multiple blizzards on a single point
    let map = map
        .into_iter()
        .filter_map(|(p, t)| {
            let t = match t {
                Raw::Clear => return None,
                Raw::Wall => Tile::Wall,
                Raw::Blizzard(bz) => Tile::Blizzard(vec![bz]),
            };
            Some((p, t))
        })
        .collect();

    (start, end, map)
}

fn default_input() -> (Vector2, Vector2, HashMap<Vector2, Tile>) {
    parse_input(include_str!("input/24.txt"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Blizzard(Vec<Vector2>),
}

/// Returns the next blizzard state for the given blizzard state.
fn next_state(map: &HashMap<Vector2, Tile>, bounds: [i64; 4]) -> HashMap<Vector2, Tile> {
    let [min_x, max_x, min_y, max_y] = bounds;
    let mut next: HashMap<Vector2, Tile> = HashMap::new();
    for (&p, t) in map {
        match t {
            Tile::Wall => {
                next.insert(p, Tile::Wall);
            }
            Tile::Blizzard(bs) => {
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
                    match next.entry(n).or_insert_with(|| Tile::Blizzard(Vec::new())) {
                        Tile::Blizzard(bs) => bs.push(b),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    next
}

/// Generates all blizzards until there is a cycle.
fn blizzards(map: HashMap<Vector2, Tile>) -> (Vec<HashMap<Vector2, Tile>>, [i64; 4]) {
    let (min_x, max_x) = map.keys().map(|p| p.x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|p| p.y).minmax().into_option().unwrap();
    let bounds = [min_x, max_x, min_y, max_y];
    let mut blizzards: Vec<HashMap<Vector2, Tile>> = vec![map];
    loop {
        let next = next_state(&blizzards[blizzards.len() - 1], bounds);
        if blizzards[0] == next {
            break;
        }
        blizzards.push(next);
    }
    (blizzards, bounds)
}

/// Finds the shortest path across the map
fn shortest(
    blizzards: &[HashMap<Vector2, Tile>],
    bounds: [i64; 4],
    start: Vector2,
    end: Vector2,
    tick: usize,
) -> usize {
    use image::{ImageBuffer, Rgb};

    let [min_x, max_x, min_y, max_y] = bounds;
    let mut q = VecDeque::from([(vec![start], tick)]);
    let mut visited = HashSet::new();
    while let Some((path, tick)) = q.pop_front() {
        let pos = path.last().copied().unwrap();
        if !visited.insert((pos, tick)) {
            continue;
        }
        let b = (tick + 1) % blizzards.len();
        for d in vectors![[0, -1], [-1, 0], [0, 1], [1, 0]] {
            let next = pos + d;
            if next == end {
                // winning path, generate images
                let dir = std::path::PathBuf::from_iter([
                    env!("CARGO_WORKSPACE_DIR"),
                    "target",
                    "visual",
                ]);
                std::fs::create_dir_all(&dir).unwrap();
                let blk = 7;
                let draw_point = |img: &mut ImageBuffer<_, _>, p: Vector2, color: Rgb<u8>| {
                    let xs = if color == Rgb([0x1e, 0x1e, 0x1e]) {
                        0..blk
                    } else {
                        1..blk
                    };
                    let ys = if color == Rgb([0x1e, 0x1e, 0x1e]) {
                        0..blk
                    } else {
                        1..blk
                    };
                    for dx in xs {
                        for dy in ys.clone() {
                            let x = blk * (p.x as u32) + dx;
                            let y = blk * (p.y as u32) + dy;
                            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                                *pixel = color;
                            } else {
                                panic!("{p:?} out of bounds -> {x},{y}");
                            }
                        }
                    }
                };

                for t in 0..(tick + 1) {
                    let b = t % blizzards.len();
                    let path = &path[..t + 1];

                    let mut img =
                        ImageBuffer::new(blk * (max_x as u32 + 1), blk * (max_y as u32 + 1));
                    for y in min_y..=max_y {
                        for x in min_x..=max_x {
                            let p = vector![x, y];
                            let color = if p == start {
                                Rgb([0x00, 0xc6, 0x00])
                            } else if path.contains(&p) {
                                Rgb([0x00, 0xc6, 0x00])
                            } else if p == end {
                                Rgb([0xc6, 0xc6, 0x00])
                            } else {
                                match blizzards[b].get(&p) {
                                    Some(Tile::Wall) => Rgb([0x1e, 0x1e, 0x1e]),
                                    Some(Tile::Blizzard(_)) => Rgb([0xad, 0xbc, 0xd6]),
                                    None => Rgb([0xef, 0xef, 0xef]),
                                }
                            };
                            draw_point(&mut img, p, color)
                        }
                    }
                    img.save(dir.join(format!("{:04}.png", t))).unwrap();
                }

                return tick + 1;
            }
            if next.y < min_y || next.y > max_y || next.x < min_x || next.x > max_x {
                continue;
            }
            if !blizzards[b].contains_key(&next) {
                let mut path = path.clone();
                path.push(next);
                q.push_back((path, tick + 1));
            }
        }
        if !blizzards[b].contains_key(&pos) {
            let mut path = path.clone();
            path.push(pos);
            q.push_back((path, tick + 1));
        }
    }
    panic!("no path found")
}

fn part1((start, end, map): (Vector2, Vector2, HashMap<Vector2, Tile>)) -> usize {
    let (bz, bounds) = blizzards(map);
    shortest(&bz, bounds, start, end, 0)
}

fn part2((start, end, map): (Vector2, Vector2, HashMap<Vector2, Tile>)) -> usize {
    return 0;
    let (bz, bounds) = blizzards(map);
    let t1 = shortest(&bz, bounds, start, end, 0);
    let t2 = shortest(&bz, bounds, end, start, t1);
    shortest(&bz, bounds, start, end, t2)
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
