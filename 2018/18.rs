use advent::prelude::*;

fn parse_input<const N: usize>(input: &str) -> [u128; N] {
    let mut grid = [0u128; N];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let a = match c {
                '.' => OPEN,
                '|' => WOODED,
                '#' => LUMBERYARD,
                c => panic!("unexpected character `{c}`"),
            };
            set(&mut grid, x as i64, y as i64, a);
        }
    }
    grid
}

fn default_input() -> [u128; 50] {
    parse_input(include_str!("input/18.txt"))
}

const OPEN: u128 = 0b00;
const WOODED: u128 = 0b01;
const LUMBERYARD: u128 = 0b10;
const MASK: u128 = 0b11;

const CARDINALS: [[i64; 2]; 8] = [
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn get<const N: usize>(grid: &[u128; N], x: i64, y: i64) -> Option<u128> {
    if x < 0 || y < 0 || x >= (N as i64) || y >= (N as i64) {
        return None;
    }
    Some((grid[y as usize] >> (x * 2)) & MASK)
}

fn set<const N: usize>(grid: &mut [u128; N], x: i64, y: i64, a: u128) {
    let acre = a << (x * 2);
    let mask = MASK << (x * 2);
    grid[y as usize] = (grid[y as usize] & !mask) | acre;
}

fn count(iter: impl Iterator<Item = u128>) -> (usize, usize) {
    let mut wooded = 0;
    let mut lumberyards = 0;
    for a in iter {
        match a {
            WOODED => wooded += 1,
            LUMBERYARD => lumberyards += 1,
            _ => {}
        }
    }
    (wooded, lumberyards)
}

fn next<const N: usize>(grid: &[u128; N]) -> [u128; N] {
    let mut next = *grid;
    for (x, y) in iproduct!(0..(N as i64), 0..(N as i64)) {
        let adjacents = CARDINALS
            .iter()
            .filter_map(|[dx, dy]| get(grid, x + dx, y + dy));
        let (wooded, lumberyards) = count(adjacents);
        let a = match get(grid, x, y).unwrap() {
            OPEN if wooded >= 3 => WOODED,
            WOODED if lumberyards >= 3 => LUMBERYARD,
            LUMBERYARD if lumberyards < 1 || wooded < 1 => OPEN,
            _ => continue,
        };
        set(&mut next, x, y, a);
    }
    next
}

fn resource_value<const N: usize>(grid: &[u128; N]) -> usize {
    let all = iproduct!(0..(N as i64), 0..(N as i64)).map(|(x, y)| get(grid, x, y).unwrap());
    let (wooded, lumberyards) = count(all);
    wooded * lumberyards
}

fn part1<const N: usize>(mut grid: [u128; N]) -> usize {
    for _ in 0..10 {
        grid = next(&grid);
    }
    resource_value(&grid)
}

fn part2<const N: usize>(mut grid: [u128; N]) -> usize {
    let n = 1_000_000_000;
    let mut store = HashMap::new();
    let mut values = Vec::new();
    let mut j = 0;
    let (i, j) = loop {
        values.push(resource_value(&grid));
        let next = next(&grid);
        if let Some(i) = store.insert(grid, j) {
            break (i, j);
        }
        grid = next;
        j += 1;
    };
    values[i + (n - i) % (j - i)]
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input::<10>(
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
    assert_eq!(part1(input), 355918);
    assert_eq!(part2(input), 202806);
}
