use advent::prelude::*;

fn parse_input(input: &str) -> BTreeMap<Vector2, Acre> {
    parse_map(input, |c| match c {
        '.' => Acre::Open,
        '|' => Acre::Wooded,
        '#' => Acre::Lumberyard,
        c => panic!("unexpected character `{}`", c),
    })
}

fn default_input() -> BTreeMap<Vector2, Acre> {
    parse_input(include_str!("input/18.txt"))
}

const CARDINALS: [Vector2; 8] = [
    vector![0, 1],
    vector![-1, 1],
    vector![-1, 0],
    vector![-1, -1],
    vector![0, -1],
    vector![1, -1],
    vector![1, 0],
    vector![1, 1],
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Wooded,
    Lumberyard,
}

fn count<'a, I>(iter: I) -> (usize, usize)
where
    I: Iterator<Item = &'a Acre>,
{
    let mut wooded = 0;
    let mut lumberyards = 0;
    for a in iter {
        match a {
            Acre::Wooded => wooded += 1,
            Acre::Lumberyard => lumberyards += 1,
            _ => {}
        }
    }
    (wooded, lumberyards)
}

fn next(map: &BTreeMap<Vector2, Acre>) -> BTreeMap<Vector2, Acre> {
    map.iter()
        .map(|(&p, &a)| {
            let adjacents = CARDINALS.iter().filter_map(|d| map.get(&(p + d)));
            let (wooded, lumberyards) = count(adjacents);
            let a = match a {
                Acre::Open if wooded >= 3 => Acre::Wooded,
                Acre::Wooded if lumberyards >= 3 => Acre::Lumberyard,
                Acre::Lumberyard if lumberyards < 1 || wooded < 1 => Acre::Open,
                same => same,
            };
            (p, a)
        })
        .collect()
}

fn resource_value(map: &BTreeMap<Vector2, Acre>) -> usize {
    let (wooded, lumberyards) = count(map.values());
    wooded * lumberyards
}

fn part1(mut map: BTreeMap<Vector2, Acre>) -> usize {
    for _ in 0..10 {
        map = next(&map);
    }
    resource_value(&map)
}

fn part2(mut map: BTreeMap<Vector2, Acre>) -> usize {
    let n = 1_000_000_000;
    let mut store = HashMap::new();
    let mut values = Vec::new();
    let mut j = 0;
    let (i, j) = loop {
        values.push(resource_value(&map));
        let next = next(&map);
        if let Some(i) = store.insert(map, j) {
            break (i, j);
        }
        map = next;
        j += 1;
    };

    values[i + (n - i) % (j - i)]
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
    );
    assert_eq!(part1(input), 1147);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 355918);
    assert_eq!(part2(input), 202806);
}
