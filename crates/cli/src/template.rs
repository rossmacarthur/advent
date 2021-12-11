fn parse_input(input: &str) -> Vec<i64> {
    todo!()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/{day}.txt"))
}

fn part1(input: Vec<i64>) -> i64 {
    todo!()
}

fn part2(input: Vec<i64>) -> i64 {
    todo!()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
