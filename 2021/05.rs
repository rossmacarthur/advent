use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" -> ").map(|s| {
                s.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .next_array::<2>()
                    .map(Vector2::from)
                    .unwrap()
            });
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2)> {
    parse_input(include_str!("input/05.txt"))
}

fn solve(input: impl Iterator<Item = (Vector2, Vector2)>) -> usize {
    input
        .flat_map(|(p1, p2)| {
            let d = (p2 - p1).map(i64::signum);
            iter::successors(Some(p1), move |&p| (p != p2).then(|| p + d))
        })
        .fold(HashMap::new(), |mut map, p| {
            *map.entry(p).or_insert(0) += 1;
            map
        })
        .into_values()
        .filter(|&count| count >= 2)
        .count()
}

fn part1(input: Vec<(Vector2, Vector2)>) -> usize {
    solve(
        input
            .into_iter()
            .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y),
    )
}

fn part2(input: Vec<(Vector2, Vector2)>) -> usize {
    solve(input.into_iter())
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
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7269);
    assert_eq!(part2(input), 21140);
}
