use advent::prelude::*;

fn parse_input(input: &str) -> (Vector2, HashMap<Vector2, Tile>) {
    let start = vector![input.find('S').expect("start") as i64, 0];
    let map = parse_map(input, |c| match c {
        '.' | 'S' => Tile::Empty,
        '^' => Tile::Splitter,
        c => panic!("unexpected character `{c}`"),
    });
    (start, map)
}

fn default_input() -> (Vector2, HashMap<Vector2, Tile>) {
    parse_input(include_input!(2025 / 07))
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Splitter,
}

const SOUTH: Vector2 = vector![0, 1];
const WEST: Vector2 = vector![-1, 0];
const EAST: Vector2 = vector![1, 0];

fn part1((start, map): (Vector2, HashMap<Vector2, Tile>)) -> i64 {
    let mut beams = HashSet::from_iter([start]);
    let mut splits = 0;
    loop {
        let next: HashSet<_> = beams
            .iter()
            .filter_map(|&beam| {
                let p = beam + SOUTH;
                let next = match map.get(&p)? {
                    Tile::Empty => Either::Left([p]),
                    Tile::Splitter => {
                        splits += 1;
                        Either::Right([p + EAST, p + WEST])
                    }
                };
                Some(next.into_iter())
            })
            .flatten()
            .collect();
        if next.is_empty() {
            break;
        }
        beams = next;
    }
    splits
}

fn part2((start, map): (Vector2, HashMap<Vector2, Tile>)) -> usize {
    let mut beams = HashMap::from_iter([(start, 1)]);
    loop {
        let next = beams
            .iter()
            .filter_map(|(&beam, &t)| {
                let p = beam + SOUTH;
                let next = match map.get(&p)? {
                    Tile::Empty => Either::Left([(p, t)]),
                    Tile::Splitter => Either::Right([(p + EAST, t), (p + WEST, t)]),
                };
                Some(next.into_iter())
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, (p, timelines)| {
                *acc.entry(p).or_insert(0) += timelines;
                acc
            });
        if next.is_empty() {
            break;
        }
        beams = next;
    }
    beams.into_values().sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    );
    assert_eq!(part1(input.clone()), 21);
    assert_eq!(part2(input), 40);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1539);
    assert_eq!(part2(input), 6479180385864);
}
