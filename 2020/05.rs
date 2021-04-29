use itertools::Itertools;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            let as_binary: String = line
                .chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => panic!("unrecognized input"),
                })
                .collect();
            let row = u64::from_str_radix(&as_binary[..7], 2).unwrap();
            let col = u64::from_str_radix(&as_binary[7..], 2).unwrap();
            row * 8 + col
        })
        .collect()
}

fn default_input() -> Vec<u64> {
    parse_input(include_str!("input/05.txt"))
}

fn part1(input: &[u64]) -> u64 {
    input.iter().max().copied().unwrap()
}

fn part2(input: &[u64]) -> u64 {
    let mut ids = input.to_vec();
    ids.sort_unstable();
    for (curr, next) in ids.iter().tuple_windows() {
        if next - curr > 1 {
            return next - 1;
        }
    }
    unreachable!()
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
    let input = parse_input("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");
    assert_eq!(part1(&input), 820);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 883);
    assert_eq!(part2(&input), 532);
}
