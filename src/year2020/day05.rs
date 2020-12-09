const INPUT: &str = include_str!("input/day05.txt");

pub fn default_input() -> Vec<u64> {
    INPUT
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

pub fn part1(input: &[u64]) -> Option<u64> {
    input.iter().max().map(|x| *x)
}

pub fn part2(input: &[u64]) -> Option<u64> {
    let mut ids = input.to_vec();
    ids.sort_unstable();
    let mut windows = ids.windows(2);
    while let Some(&[prev, next]) = windows.next() {
        if next - prev > 1 {
            return Some(next - 1);
        }
    }
    None
}
