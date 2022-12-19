use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, Tile> {
    parse_map(input, |c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Cavern,
        'G' => Tile::Unit(Unit {
            team: Team::Goblin,
            hp: 200,
            power: 3,
        }),
        'E' => Tile::Unit(Unit {
            team: Team::Elf,
            hp: 200,
            power: 3,
        }),
        c => panic!("unexpected character `{c}`"),
    })
}

fn default_input() -> HashMap<Vector2, Tile> {
    parse_input(include_str!("input/15.txt"))
}

const NORTH: Vector2 = vector![0, -1];
const WEST: Vector2 = vector![-1, 0];
const SOUTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, WEST, EAST, SOUTH];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Cavern,
    Unit(Unit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Unit {
    team: Team,
    hp: i32,
    power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Team {
    Goblin,
    Elf,
}

enum Outcome {
    Continue,
    Break,
    ElfDied,
}

fn try_move(cave: &HashMap<Vector2, Tile>, pos: Vector2, team: Team) -> Option<Vector2> {
    let mut min = Vec::new();
    let mut q = VecDeque::from([(0, None, pos)]);
    let mut visited = HashSet::new();
    while let Some((dist, first, pos)) = q.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        for d in CARDINALS {
            let next = pos + d;
            let first = first.or(Some(next));
            match cave.get(&next) {
                Some(Tile::Cavern) => {
                    q.push_back((dist + 1, first, next));
                }
                Some(Tile::Unit(u)) if u.team != team => {
                    min.push((dist + 1, first, next));
                }
                _ => {}
            }
        }
    }
    min.into_iter()
        .min_by_key(|(dist, _, p)| (*dist, p.y, p.x))
        .map(|(_, next, _)| next.unwrap())
}

fn play_turn(cave: &mut HashMap<Vector2, Tile>, mut pos: Vector2) -> Outcome {
    // Relookup up the unit, in case it has died.
    let unit = match *cave.get(&pos).unwrap() {
        Tile::Unit(u) => u,
        _ => return Outcome::Continue,
    };

    // Each unit begins its turn by identifying all possible enemy units.
    let targets: Vec<_> = cave
        .iter()
        .filter_map(|(&p, t)| match t {
            Tile::Unit(u) if u.team != unit.team => Some(p),
            _ => None,
        })
        .collect();

    // If no targets remain, combat ends.
    if targets.is_empty() {
        return Outcome::Break;
    }

    // Check whether we are in range of any enemy.
    let in_range = targets
        .into_iter()
        .flat_map(|p| CARDINALS.iter().map(move |d| p + d))
        .any(|p| p == pos);

    if !in_range {
        // If we're not in range then try to move to the closest enemy
        if let Some(next) = try_move(cave, pos, unit.team) {
            cave.insert(next, Tile::Unit(unit));
            cave.insert(pos, Tile::Cavern);
            pos = next;
        }
    }

    // Find all enemies in the immediate range.
    let mut enemies = CARDINALS
        .iter()
        .map(|d| pos + d)
        .filter_map(|p| match cave.get(&p).unwrap() {
            Tile::Unit(u) if u.team != unit.team => Some((p, u)),
            _ => None,
        })
        .sorted_by_key(|(p, u)| (u.hp, p.y, p.x));

    // If there is one, then attack it!
    if let Some((enemy, _)) = enemies.next() {
        let enemy = cave.get_mut(&enemy).unwrap();
        let u = match enemy {
            Tile::Unit(u) => u,
            _ => unreachable!(),
        };
        u.hp -= unit.power;
        if u.hp <= 0 {
            if u.power > 3 && u.team == Team::Elf {
                return Outcome::ElfDied;
            }
            *enemy = Tile::Cavern;
        }
    }

    Outcome::Continue
}

fn play_round(cave: &mut HashMap<Vector2, Tile>) -> Outcome {
    let units = cave
        .iter()
        .filter_map(|(&p, t)| match t {
            Tile::Unit(_) => Some(p),
            _ => None,
        })
        .sorted_by_key(|p| (p.y, p.x));

    for pos in units {
        match play_turn(cave, pos) {
            Outcome::Continue => continue,
            o => return o,
        }
    }
    Outcome::Continue
}

fn play(mut cave: HashMap<Vector2, Tile>) -> Option<i32> {
    let mut round: i32 = 0;
    loop {
        match play_round(&mut cave) {
            Outcome::Continue => round += 1,
            Outcome::Break => break,
            Outcome::ElfDied => return None,
        }
    }

    let hps: i32 = cave
        .values()
        .map(|&t| match t {
            Tile::Unit(u) => u.hp,
            _ => 0,
        })
        .sum();
    Some(round * hps)
}

fn part1(cave: HashMap<Vector2, Tile>) -> i32 {
    play(cave).unwrap()
}

fn part2(cave: HashMap<Vector2, Tile>) -> i32 {
    (4..)
        .find_map(|power| {
            let mut cave = cave.clone();
            for t in cave.values_mut() {
                match t {
                    Tile::Unit(u) if u.team == Team::Elf => u.power = power,
                    _ => {}
                }
            }
            play(cave)
        })
        .unwrap()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
    );
    assert_eq!(part1(input.clone()), 27730);
    assert_eq!(part2(input), 4988);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"#,
    );
    assert_eq!(part1(input.clone()), 36334);
    assert_eq!(part2(input), 29064);
}

#[test]
fn example3() {
    let input = parse_input(
        r#"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#,
    );
    assert_eq!(part1(input.clone()), 39514);
    assert_eq!(part2(input), 31284)
}

#[test]
fn example4() {
    let input = parse_input(
        r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#,
    );
    assert_eq!(part1(input.clone()), 27755);
    assert_eq!(part2(input), 3478);
}

#[test]
fn example5() {
    let input = parse_input(
        r#"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#,
    );
    assert_eq!(part1(input.clone()), 28944);
    assert_eq!(part2(input), 6474);
}

#[test]
fn example6() {
    let input = parse_input(
        r#"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"#,
    );
    assert_eq!(part1(input.clone()), 18740);
    assert_eq!(part2(input), 1140);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 201638);
    assert_eq!(part2(input), 95764);
}
