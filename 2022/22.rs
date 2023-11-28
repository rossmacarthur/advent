use advent::prelude::*;

fn parse_input(input: &str) -> (HashMap<Vector2, Tile>, Vec<Instr>) {
    let (map, path) = input.split_once("\n\n").unwrap();

    let raw: HashMap<_, _> = parse_map(map, |c| match c {
        ' ' => None,
        '#' => Some(Tile::Wall),
        '.' => Some(Tile::Open),
        _ => panic!("unknown character `{c}`"),
    });

    let map = raw
        .into_iter()
        .filter_map(|(p, t)| t.map(|t| (p, t)))
        .collect();

    let instrs: Vec<_> = regex!(r"\d+|[RL]")
        .captures_iter(path)
        .map(|caps| match &caps[0] {
            "L" => Instr::TurnLeft,
            "R" => Instr::TurnRight,
            n => Instr::Move(n.parse().unwrap()),
        })
        .collect();

    (map, instrs)
}

fn default_input() -> (HashMap<Vector2, Tile>, Vec<Instr>) {
    parse_input(include_input!(2022 / 22))
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    TurnLeft,
    TurnRight,
    Move(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

use Direction::*;

impl Direction {
    fn vector(self) -> Vector2 {
        match self {
            Right => vector![1, 0],
            Down => vector![0, 1],
            Left => vector![-1, 0],
            Up => vector![0, -1],
        }
    }
}

fn wrap(map: &HashMap<Vector2, Tile>, mut pos: Vector2, facing: Direction) -> (Vector2, Direction) {
    let d = facing.vector();
    while map.contains_key(&(pos - d)) {
        pos -= d
    }
    (pos, facing)
}

fn wrap_cube(_: &HashMap<Vector2, Tile>, pos: Vector2, d: Direction) -> (Vector2, Direction) {
    let x = pos.x;
    let y = pos.y;
    if d == Up && y == 0 && (50..100).contains(&x) {
        return (vector![0, x + 100], Right);
    }
    if d == Left && x == 0 && (150..200).contains(&y) {
        return (vector![y - 100, 0], Down);
    }
    if d == Up && y == 0 && (100..150).contains(&x) {
        return (vector![x - 100, 199], Up);
    }
    if d == Down && y == 199 && (0..50).contains(&x) {
        return (vector![x + 100, 0], Down);
    }
    if d == Left && x == 50 && (0..50).contains(&y) {
        return (vector![0, 149 - y], Right);
    }
    if d == Left && x == 0 && (100..150).contains(&y) {
        return (vector![50, 149 - y], Right);
    }
    if d == Right && x == 149 && (0..50).contains(&y) {
        return (vector![99, 149 - y], Left);
    }
    if d == Right && x == 99 && (100..150).contains(&y) {
        return (vector![149, 149 - y], Left);
    }
    if d == Down && y == 49 && (100..150).contains(&x) {
        return (vector![99, x - 50], Left);
    }
    if d == Right && x == 99 && (50..100).contains(&y) {
        return (vector![y + 50, 49], Up);
    }
    if d == Left && x == 50 && (50..100).contains(&y) {
        return (vector![y - 50, 100], Down);
    }
    if d == Up && y == 100 && (0..50).contains(&x) {
        return (vector![50, x + 50], Right);
    }
    if d == Down && y == 149 && (50..100).contains(&x) {
        return (vector![49, x + 100], Left);
    }
    if d == Right && x == 49 && (150..200).contains(&y) {
        return (vector![y - 100, 149], Up);
    }
    (pos, d)
}

fn solve<W>(map: HashMap<Vector2, Tile>, instrs: Vec<Instr>, wrap: W) -> i64
where
    W: Fn(&HashMap<Vector2, Tile>, Vector2, Direction) -> (Vector2, Direction),
{
    let start_x = map
        .iter()
        .filter_map(|(v, t)| (v.y == 0 && matches!(t, Tile::Open)).some(v.x))
        .min()
        .unwrap();

    let (mut pos, mut facing) = (vector![start_x, 0], Right);
    for instr in instrs {
        match instr {
            Instr::TurnLeft => {
                facing = match facing {
                    Right => Up,
                    Down => Right,
                    Left => Down,
                    Up => Left,
                }
            }
            Instr::TurnRight => {
                facing = match facing {
                    Right => Down,
                    Down => Left,
                    Left => Up,
                    Up => Right,
                }
            }
            Instr::Move(steps) => {
                for _ in 0..steps {
                    let next = pos + facing.vector();
                    match map.get(&next) {
                        Some(Tile::Wall) => break,
                        Some(Tile::Open) => pos = next,
                        None => {
                            let (next, next_facing) = wrap(&map, pos, facing);
                            match &map[&next] {
                                Tile::Open => (pos, facing) = (next, next_facing),
                                Tile::Wall => break,
                            }
                        }
                    }
                }
            }
        }
    }
    (facing as i64) + 1000 * (pos.y + 1) + 4 * (pos.x + 1)
}

fn part1((map, instrs): (HashMap<Vector2, Tile>, Vec<Instr>)) -> i64 {
    solve(map, instrs, wrap)
}

fn part2((map, instrs): (HashMap<Vector2, Tile>, Vec<Instr>)) -> i64 {
    solve(map, instrs, wrap_cube)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
",
    );
    assert_eq!(part1(input), 6032);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 189140);
    assert_eq!(part2(input), 115063);
}

#[test]
fn test_wrap_cube() {
    let inv = |d| match d {
        Right => Left,
        Down => Up,
        Left => Right,
        Up => Down,
    };
    let (map, _) = default_input();
    for &p in map.keys().sorted() {
        for d in [Right, Down, Left, Up] {
            let (np, nd) = wrap_cube(&map, p, d);
            if (np, nd) == (p, d) {
                continue;
            }
            let nd_back = inv(nd);
            let (rp, rd_back) = wrap_cube(&map, np, nd_back);
            let rd = inv(rd_back);
            assert_eq!((p, d), (rp, rd));
        }
    }
}
