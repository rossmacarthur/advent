use std::hash::Hash;

use advent::prelude::*;

fn default_input() -> (i64, Vector2) {
    (3558, vector![15, 740])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Region {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool {
    Neither = 0,
    Torch = 1,
    ClimbingGear = 2,
}

const UP: Vector2 = vector![0, -1];
const LEFT: Vector2 = vector![-1, 0];
const DOWN: Vector2 = vector![0, 1];
const RIGHT: Vector2 = vector![1, 0];
const DIRECTIONS: [Vector2; 4] = [UP, DOWN, LEFT, RIGHT];

fn region(erosion_level: i64) -> Region {
    match erosion_level.rem_euclid(3) {
        0 => Region::Rocky,
        1 => Region::Wet,
        2 => Region::Narrow,
        _ => unreachable!(),
    }
}

fn can_use(region: Region, tool: Tool) -> bool {
    // This works because the enum values for `Region` and `Tool`
    // are setup such that that the value for `Tool` has the same
    // enum value of the `Region` that it *cannot* be used on.
    region as u8 != tool as u8
}

fn build_map(depth: i64, target: Vector2, k: i64) -> HashMap<Vector2, Region> {
    let mut map: HashMap<Vector2, (i64, Region)> = HashMap::new();
    let end = target * k;
    for y in 0..=end.y {
        for x in 0..=end.x {
            let pos = vector![x, y];
            let geologic_index = if pos == vector![0, 0] || pos == target {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                map[&vector![x - 1, y]].0 * map[&vector![x, y - 1]].0
            };
            let erosion_level = (geologic_index + depth).rem_euclid(20183);
            let region = region(erosion_level);
            map.insert(pos, (erosion_level, region));
        }
    }

    map.into_iter().map(|(p, (_, t))| (p, t)).collect()
}

fn part1((depth, target): (i64, Vector2)) -> usize {
    build_map(depth, target, 1)
        .values()
        .map(|&region| region as usize)
        .sum()
}

fn part2((depth, target): (i64, Vector2)) -> usize {
    let map = build_map(depth, target, 2);

    let mut pq = BinaryHeap::from_iter([(Reverse(0usize), vector![0, 0], Tool::Torch)]);
    let mut visited = HashSet::new();

    while let Some((Reverse(time), pos, equipped)) = pq.pop() {
        if pos == target && equipped == Tool::Torch {
            return time;
        }

        if !visited.insert((pos, equipped)) {
            continue;
        }

        // Push all possible tool changes onto the queue, the cost for changing
        // the tool is 7 minutes.
        let region = map[&pos];
        for tool in [Tool::Neither, Tool::Torch, Tool::ClimbingGear] {
            if tool != equipped && can_use(region, tool) {
                pq.push((Reverse(time + 7), pos, tool));
            }
        }

        // Go in all possible adjacent directions with the current tool, the
        // cost for moving is 1 minute.
        for d in DIRECTIONS {
            let next = pos + d;
            if matches!(map.get(&next), Some(&region) if can_use(region, equipped)) {
                pq.push((Reverse(time + 1), next, equipped));
            }
        }
    }

    panic!("no path found")
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = (510, vector![10, 10]);
    assert_eq!(part1(input), 114);
    assert_eq!(part2(input), 45);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 11810);
    assert_eq!(part2(input), 1015);
}
