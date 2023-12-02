use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(i64, Vec<Vector3>)> {
    input
        .lines()
        .map(|line| {
            let (id, sets) = line.split_once(": ").unwrap();
            let id = id.strip_prefix("Game ").unwrap().parse().unwrap();
            let sets = sets
                .split("; ")
                .map(|set| {
                    set.split(", ").fold(Vector3::zero(), |mut set, bunch| {
                        let (count, color) = bunch.split_once(' ').unwrap();
                        let count: i64 = count.parse().unwrap();
                        match color {
                            "red" => set.x += count,
                            "green" => set.y += count,
                            "blue" => set.z += count,
                            _ => panic!("unexpected color `{color}`"),
                        };
                        set
                    })
                })
                .collect();
            (id, sets)
        })
        .collect()
}

fn default_input() -> Vec<(i64, Vec<Vector3>)> {
    parse_input(include_input!(2023 / 02))
}

fn part1(games: Vec<(i64, Vec<Vector3>)>) -> i64 {
    games
        .into_iter()
        .filter_map(|(id, sets)| {
            sets.into_iter()
                .all(|set| set.x <= 12 && set.y <= 13 && set.z <= 14)
                .some(id)
        })
        .sum()
}

fn part2(games: Vec<(i64, Vec<Vector3>)>) -> i64 {
    games
        .into_iter()
        .map(|(_, sets)| {
            let max = sets
                .into_iter()
                .reduce(|acc, set| vector![acc.x.max(set.x), acc.y.max(set.y), acc.z.max(set.z)])
                .unwrap();
            max.into_iter().product::<i64>()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    );
    assert_eq!(part1(input.clone()), 8);
    assert_eq!(part2(input), 2286);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1734);
    assert_eq!(part2(input), 70387);
}
