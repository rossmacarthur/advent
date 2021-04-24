use std::collections::HashMap;

pub fn default_input() -> Vec<usize> {
    vec![2, 20, 0, 4, 1, 17]
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

pub fn part1(input: &[usize]) -> usize {
    count(input, 2020)
}

pub fn part2(input: &[usize]) -> usize {
    count(input, 30000000)
}
