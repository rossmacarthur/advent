use advent::prelude::*;

fn parse_input(input: &str) -> (HashMap<Vector2, Tile>, Vec<Vector2>) {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = parse_map(map, |c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        'O' => Tile::Box,
        '@' => Tile::Robot,
        _ => panic!("unexpected character `{}`", c),
    });

    let moves: Vec<_> = moves
        .chars()
        .filter_map(|c| match c {
            '^' => Some(NORTH),
            'v' => Some(SOUTH),
            '<' => Some(WEST),
            '>' => Some(EAST),
            _ => None,
        })
        .collect();

    (map, moves)
}

fn default_input() -> (HashMap<Vector2, Tile>, Vec<Vector2>) {
    parse_input(include_input!(2024 / 15))
}

const NORTH: Vector2 = vector![0, -1];
const SOUTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];
const WEST: Vector2 = vector![-1, 0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

/// Converts a part 1 map to a part 2 map.
fn widen(map: HashMap<Vector2, Tile>) -> HashMap<Vector2, Tile> {
    map.into_iter()
        .flat_map(|(p, t)| {
            let p1 = vector![p.x * 2, p.y];
            let p2 = vector![p.x * 2 + 1, p1.y];
            match t {
                Tile::Empty => [(p1, Tile::Empty), (p2, Tile::Empty)],
                Tile::Wall => [(p1, Tile::Wall), (p2, Tile::Wall)],
                Tile::Robot => [(p1, Tile::Robot), (p2, Tile::Empty)],
                Tile::Box => [(p1, Tile::BoxLeft), (p2, Tile::BoxRight)],
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve(mut map: HashMap<Vector2, Tile>, moves: Vec<Vector2>) -> i64 {
    let mut robot = map
        .iter()
        .find_map(|(p, &t)| (t == Tile::Robot).some(*p))
        .unwrap();

    // Queue and visited set for BFS, defined outside the loop to reduce the
    // number of memory allocations.
    let mut q = VecDeque::new();

    // Uses a vector instead of a set because we can easily sort it by distance
    // from the robot in order to move all the boxes.
    let mut visited = Vec::new();

    'moves: for m in moves {
        // re-use allocation
        q.clear();
        visited.clear();

        // Start at the robot, then do BFS in the direction of the move to
        // figure out which boxes will move.
        q.push_back(robot);

        while let Some(p) = q.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            visited.push(p);

            let next = p + m;
            match &map[&next] {
                Tile::Empty => continue,
                Tile::Wall => continue 'moves, // nothing can move
                Tile::Box => q.push_back(next),
                Tile::BoxLeft => q.extend([next, next + EAST]),
                Tile::BoxRight => q.extend([next + WEST, next]),
                Tile::Robot => unreachable!(),
            }
        }

        // Sort it by distance from the robot and then move all the boxes.
        visited.sort_unstable_by_key(|p| ((p - robot).l1_norm()));
        for &p in visited.iter().rev() {
            map.insert(p + m, map[&p]);
            map.insert(p, Tile::Empty);
        }

        robot += m;
    }

    map.into_iter()
        .filter(|(_, t)| matches!(t, Tile::Box | Tile::BoxLeft))
        .map(|(p, _)| p.x + 100 * p.y)
        .sum()
}

fn part1((map, moves): (HashMap<Vector2, Tile>, Vec<Vector2>)) -> i64 {
    solve(map, moves)
}

fn part2((map, moves): (HashMap<Vector2, Tile>, Vec<Vector2>)) -> i64 {
    solve(widen(map), moves)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    );
    assert_eq!(part1(input.clone()), 2028);
    assert_eq!(part2(input), 1751);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
    );
    assert_eq!(part1(input.clone()), 10092);
    assert_eq!(part2(input), 9021);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1451928);
    assert_eq!(part2(input), 1462788);
}
