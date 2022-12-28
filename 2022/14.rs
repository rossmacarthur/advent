use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector2> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(str::parse)
                        .map(Result::unwrap)
                        .next_array()
                        .map(Vector2::from)
                        .unwrap()
                })
                .array_windows()
                .flat_map(|[p1, p2]| {
                    let d = (p2 - p1).map(i64::signum);
                    iter::successors(Some(p1), move |&p| (p != p2).some(p + d))
                })
        })
        .collect()
}

fn default_input() -> HashSet<Vector2> {
    parse_input(include_str!("input/14.txt"))
}

fn solve(mut cave: HashSet<Vector2>, part2: bool) -> usize {
    let mut max_y = cave.iter().map(|v| v.y).max().unwrap();
    if part2 {
        max_y += 2;
    }
    let mut flowing = vec![vector![500, 0]];
    let mut count = 0;
    while let Some(sand) = flowing.last().copied() {
        // Check if there is air in any of the following directions
        match vectors!([0, 1], [-1, 1], [1, 1])
            .into_iter()
            .map(|d| sand + d)
            .find_map(|p| (!cave.contains(&p)).some(p))
        {
            // There is space for the sand to flow, so update the sand position
            // and continue flowing
            Some(p) if p.y < max_y => flowing.push(p),
            // We've gone out of bounds, if part 1 then we're done
            Some(_) if !part2 => break,
            // Otherwise we can no longer flow, so we just insert sand at this
            // location
            _ => {
                cave.insert(sand);
                flowing.pop();
                count += 1;
            }
        }
    }
    count
}

fn part1(cave: HashSet<Vector2>) -> usize {
    solve(cave, false)
}

fn part2(cave: HashSet<Vector2>) -> usize {
    solve(cave, true)
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
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    assert_eq!(part1(input.clone()), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 774);
    assert_eq!(part2(input), 22499);
}
