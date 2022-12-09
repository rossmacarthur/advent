use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, i64> {
    parse_map(input, |c| match c {
        c @ '0'..='9' => c as i64 - b'0' as i64,
        c => panic!("unexpected character `{}`", c),
    })
}

fn default_input() -> HashMap<Vector2, i64> {
    parse_input(include_str!("input/09.txt"))
}

const NORTH: Vector2 = vector![0, -1];
const SOUTH: Vector2 = vector![0, 1];
const WEST: Vector2 = vector![-1, 0];
const EAST: Vector2 = vector![1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, SOUTH, WEST, EAST];

fn low_points(map: &HashMap<Vector2, i64>) -> impl Iterator<Item = (Vector2, i64)> + '_ {
    map.iter()
        .filter(|(p, height)| {
            CARDINALS
                .iter()
                .filter_map(|d| map.get(&(*p + d)))
                .all(|h| h > height)
        })
        .map(|(p, h)| (*p, *h))
}

fn part1(map: HashMap<Vector2, i64>) -> i64 {
    low_points(&map).map(|(_, h)| h + 1).sum()
}

fn part2(map: HashMap<Vector2, i64>) -> usize {
    let mut basins = Vec::new();
    for (p, _) in low_points(&map) {
        let mut q = VecDeque::from([p]);
        let mut visited = HashSet::new();
        while let Some(p) = q.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            for d in CARDINALS {
                let next = p + d;
                if let Some(0..=8) = map.get(&next) {
                    q.push_back(next);
                }
            }
        }
        basins.push(visited.len());
    }
    basins.sort_unstable();
    basins.into_iter().rev().take(3).product()
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
        "\
2199943210
3987894921
9856789892
8767896789
9899965678",
    );
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input), 1134);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 423);
    assert_eq!(part2(input), 1198704);
}
