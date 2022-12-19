use advent::prelude::*;

fn parse_input(s: &str) -> Vec<Vec<Vector2>> {
    s.lines()
        .map(|line| {
            regex!(r"(e|se|sw|w|nw|ne)")
                .find_iter(line)
                .map(|cap| match cap.as_str() {
                    "e" => [2, 0],
                    "se" => [1, -1],
                    "sw" => [-1, -1],
                    "w" => [-2, 0],
                    "nw" => [-1, 1],
                    "ne" => [1, 1],
                    d => panic!("unexpected direction `{d}`"),
                })
                .map(Vector2::from)
                .collect()
        })
        .collect()
}

fn default_input() -> Vec<Vec<Vector2>> {
    parse_input(include_str!("input/24.txt"))
}

fn neighbours(center: Vector2) -> Vec<Vector2> {
    [[2, 0], [1, -1], [-1, -1], [-2, 0], [-1, 1], [1, 1]]
        .iter()
        .copied()
        .map(Vector2::from)
        .map(|direction| center + direction)
        .collect()
}

fn black_neighbours(state: &HashSet<Vector2>, center: Vector2) -> usize {
    neighbours(center)
        .into_iter()
        .filter_map(|vector| state.get(&vector))
        .count()
}

fn initial_state(input: Vec<Vec<Vector2>>) -> HashSet<Vector2> {
    let mut state = HashSet::new();
    for path in input {
        let location = path.into_iter().sum();
        if state.contains(&location) {
            state.remove(&location);
        } else {
            state.insert(location);
        }
    }
    state
}

fn next_state(state: HashSet<Vector2>) -> HashSet<Vector2> {
    state
        .iter()
        .copied()
        .flat_map(neighbours)
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&vector| {
            let black = black_neighbours(&state, vector);
            match state.contains(&vector) {
                true if black == 0 || black > 2 => false,
                false if black == 2 => true,
                same => same,
            }
        })
        .collect()
}

fn part1(input: Vec<Vec<Vector2>>) -> usize {
    initial_state(input).len()
}

fn part2(input: Vec<Vec<Vector2>>) -> usize {
    let mut state = initial_state(input);
    for _ in 0..100 {
        state = next_state(state)
    }
    state.len()
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
        "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
    );
    assert_eq!(part1(input.clone()), 10);
    assert_eq!(part2(input), 2208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 438);
    assert_eq!(part2(input), 4038);
}
