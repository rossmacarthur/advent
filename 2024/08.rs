use std::ops::RangeInclusive;

use advent::prelude::*;

fn parse_input(input: &str) -> (Bounds, HashMap<Vector2, char>) {
    let mut map: HashMap<_, _> = parse_map(input, |c| c);
    let (min_x, max_x) = map.keys().map(|p| p.x).min_max().unwrap();
    let (min_y, max_y) = map.keys().map(|p| p.y).min_max().unwrap();
    map.retain(|_, c| *c != '.');
    let bounds = Bounds {
        x: min_x..=max_x,
        y: min_y..=max_y,
    };
    (bounds, map)
}

fn default_input() -> (Bounds, HashMap<Vector2, char>) {
    parse_input(include_input!(2024 / 08))
}

#[derive(Debug, Clone)]
struct Bounds {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl Bounds {
    fn contains(&self, p: Vector2) -> bool {
        self.x.contains(&p.x) && self.y.contains(&p.y)
    }
}

fn solve<I, F>(map: HashMap<Vector2, char>, f: F) -> usize
where
    I: Iterator<Item = Vector2>,
    F: FnMut(Vec<Vector2>) -> I,
{
    map.iter()
        .fold(HashMap::new(), |mut acc, (&p, &c)| {
            acc.entry(c).or_insert_with(Vec::new).push(p);
            acc
        })
        .into_values()
        .flat_map(f)
        .collect::<HashSet<_>>()
        .len()
}

fn part1((bounds, map): (Bounds, HashMap<Vector2, char>)) -> usize {
    solve(map, |ps| {
        ps.into_iter()
            .array_combinations()
            .flat_map(|[p1, p2]| [p1 * 2 - p2, p2 * 2 - p1])
            .filter(|&p| bounds.contains(p))
    })
}

fn part2((bounds, map): (Bounds, HashMap<Vector2, char>)) -> usize {
    solve(map, |ps| {
        ps.into_iter().array_combinations().flat_map(|[p1, p2]| {
            let d = p2 - p1;
            let neg = (0..)
                .map(move |i| p1 - d * i)
                .take_while(|&p| bounds.contains(p));
            let pos = (0..)
                .map(move |i| p2 + d * i)
                .take_while(|&p| bounds.contains(p));
            neg.chain(pos)
        })
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    );
    assert_eq!(part1(input.clone()), 14);
    assert_eq!(part2(input), 34);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 293);
    assert_eq!(part2(input), 934);
}
