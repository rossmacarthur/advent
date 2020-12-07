const INPUT: &str = include_str!("input/day05.txt");

pub fn default_input() -> Vec<usize> {
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
            let row = usize::from_str_radix(&as_binary[..7], 2).unwrap();
            let col = usize::from_str_radix(&as_binary[7..], 2).unwrap();
            row * 8 + col
        })
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

pub fn part2(input: &[usize]) -> usize {
    let mut ids = input.to_vec();
    ids.sort_unstable();
    let mut windows = ids.windows(2);
    while let Some(&[prev, next]) = windows.next() {
        if next - prev > 1 {
            return next - 1;
        }
    }
    panic!("failed to find missing seat ID");
}
