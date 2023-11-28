use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector2> {
    parse_map_set(input)
}

fn default_input() -> HashSet<Vector2> {
    parse_input(include_input!(2022 / 23))
}

const N: Vector2 = vector![0, -1];
const S: Vector2 = vector![0, 1];
const E: Vector2 = vector![1, 0];
const W: Vector2 = vector![-1, 0];
const NE: Vector2 = vector![1, -1];
const NW: Vector2 = vector![-1, -1];
const SE: Vector2 = vector![1, 1];
const SW: Vector2 = vector![-1, 1];

fn solve<I>(mut grove: HashSet<Vector2>, rounds: I) -> i64
where
    I: Iterator<Item = usize>,
{
    // All the possible directions from an elf
    let all = [N, S, E, W, NE, NW, SE, SW];

    // Each set of directions to consider
    let consider = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

    // All the proposed moves by elves
    let mut proposed: HashMap<Vector2, Vec<Vector2>> = HashMap::new();

    for round in rounds {
        for p in &grove {
            let is_open = |ds: &[_]| ds.iter().all(|d| !grove.contains(&(p + d)));

            if is_open(&all) {
                continue;
            }

            for &way in consider.iter().cycle().skip(round % 4).take(4) {
                if is_open(&way) {
                    proposed.entry(p + way[0]).or_default().push(*p);
                    break;
                }
            }
        }

        // Part 2, can no longer continue, return the round number
        if proposed.is_empty() {
            return (round + 1) as i64;
        }

        for (p, srcs) in proposed.drain() {
            if srcs.len() > 1 {
                continue;
            }
            grove.remove(&srcs[0]);
            grove.insert(p);
        }
    }

    // Part 1, return the rectangle size
    let min_x = grove.iter().map(|p| p.x).min().unwrap();
    let max_x = grove.iter().map(|p| p.x).max().unwrap();
    let min_y = grove.iter().map(|p| p.y).min().unwrap();
    let max_y = grove.iter().map(|p| p.y).max().unwrap();
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    area - grove.len() as i64
}

fn part1(grove: HashSet<Vector2>) -> i64 {
    solve(grove, 0..10)
}

fn part2(grove: HashSet<Vector2>) -> i64 {
    solve(grove, 0..)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
    );
    assert_eq!(part1(input.clone()), 110);
    assert_eq!(part2(input), 20);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3849);
    assert_eq!(part2(input), 995);
}
