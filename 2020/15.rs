use std::collections::HashMap;

fn default_input() -> &'static [usize] {
    &[2, 20, 0, 4, 1, 17]
}

fn count(input: &[usize], until: usize) -> usize {
    let mut map: HashMap<_, _> = input
        .iter()
        .copied()
        .enumerate()
        .map(|(turn, num)| (num, turn))
        .collect();
    let mut next = 0;
    for turn in input.len()..(until - 1) {
        next = match map.insert(next, turn) {
            Some(prev_turn) => (turn - prev_turn),
            None => 0,
        };
    }
    next
}

fn part1(input: &[usize]) -> usize {
    count(input, 2020)
}

fn part2(input: &[usize]) -> usize {
    count(input, 30000000)
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = [0, 3, 6];
    assert_eq!(part1(&input), 436);
    assert_eq!(part2(&input), 175594);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 758);
    assert_eq!(part2(&input), 814);
}
