use advent::prelude::*;

fn parse_input(input: &str) -> (Vector2, Vector2, i64, HashSet<Vector3>) {
    #[derive(Clone, Copy)]
    enum Tile {
        Clear,
        Wall,
        Blizzard([i64; 2]),
    }

    let raw: HashMap<_, _> = parse_map(input, |c| match c {
        '.' => Tile::Clear,
        '#' => Tile::Wall,
        '>' => Tile::Blizzard([1, 0]),
        '<' => Tile::Blizzard([-1, 0]),
        '^' => Tile::Blizzard([0, -1]),
        'v' => Tile::Blizzard([0, 1]),
        _ => panic!("unexpected character `{c}`"),
    });

    let max_x = raw.keys().map(|p| p.x).max().unwrap();
    let max_y = raw.keys().map(|p| p.y).max().unwrap();
    let cycle = lcm(max_x - 1, max_y - 1);

    // Find the starting and ending points
    let start = raw
        .iter()
        .find_map(|(&p, tile)| (p.y == 0 && matches!(tile, Tile::Clear)).some(p))
        .unwrap();
    let end = raw
        .iter()
        .find_map(|(&p, tile)| (p.y == max_y && matches!(tile, Tile::Clear)).some(p))
        .unwrap();

    // All points on the map
    let base: HashSet<_> = cartesian_product!(0..=max_x, 0..=max_y, 0..cycle)
        .map(|(x, y, z)| vector![x, y, z])
        .collect();

    // All unnavigatable points
    let blizzard: HashSet<_> = (0..cycle)
        .flat_map(|t| {
            raw.iter().filter_map(move |(&p, tile)| match tile {
                Tile::Clear => None,
                Tile::Wall => Some(with_z(p, t)),
                Tile::Blizzard([dx, dy]) => Some(vector![
                    (p.x + t * dx - 1).rem_euclid(max_x - 1) + 1,
                    (p.y + t * dy - 1).rem_euclid(max_y - 1) + 1,
                    t,
                ]),
            })
        })
        .collect();

    let map = base.difference(&blizzard).copied().collect();

    (start, end, cycle, map)
}

fn default_input() -> (Vector2, Vector2, i64, HashSet<Vector3>) {
    parse_input(include_input!(2022 / 24))
}

fn with_z(p: Vector2, z: i64) -> Vector3 {
    vector![p.x, p.y, z]
}

/// Finds the shortest path across the map
fn shortest(map: &HashSet<Vector3>, cycle: i64, start: Vector3, end: Vector2) -> i64 {
    let mut q = VecDeque::from([start]);
    let mut visited = HashSet::new();
    while let Some(pos) = q.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        if pos.x == end.x && pos.y == end.y {
            return pos.z;
        }
        for d in vectors!([0, 0, 1], [0, -1, 1], [-1, 0, 1], [0, 1, 1], [1, 0, 1]) {
            let next = pos + d;
            if map.contains(&vector![next.x, next.y, next.z % cycle]) {
                q.push_back(next);
            }
        }
    }
    panic!("no path found")
}

fn part1((start, end, cycle, maps): (Vector2, Vector2, i64, HashSet<Vector3>)) -> i64 {
    shortest(&maps, cycle, with_z(start, 0), end)
}

fn part2((start, end, cycle, maps): (Vector2, Vector2, i64, HashSet<Vector3>)) -> i64 {
    let t1 = shortest(&maps, cycle, with_z(start, 0), end);
    let t2 = shortest(&maps, cycle, with_z(end, t1), start);
    shortest(&maps, cycle, with_z(start, t2), end)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
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
