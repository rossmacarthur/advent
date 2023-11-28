use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2018 / 08))
}

fn parse(mut data: &[usize]) -> (&[usize], usize, usize) {
    // The number of children in this node.
    let nc = data[0];
    // The number of metadata entries in this node.
    let nm = data[1];
    // The remaining data in this node.
    data = &data[2..];

    let mut total = 0;
    let mut values = Vec::new();

    // Parse each child node using recursion.
    for _ in 0..nc {
        let (d, t, v) = parse(data);
        data = d;
        total += t;
        values.push(v)
    }

    // The metadata entries.
    let metas = &data[..nm];

    // The remaining data. This is the next child node so we just return it back
    // to the caller to parse.
    data = &data[nm..];

    let sum: usize = metas.iter().sum();
    total += sum;

    if nc == 0 {
        (data, total, sum)
    } else {
        let value = metas.iter().filter_map(|i| values.get(i - 1)).sum();
        (data, total, value)
    }
}

fn part1(data: Vec<usize>) -> usize {
    let (_, total, _) = parse(&data);
    total
}

fn part2(data: Vec<usize>) -> usize {
    let (_, _, value) = parse(&data);
    value
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(part1(input.clone()), 138);
    assert_eq!(part2(input), 66);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 45868);
    assert_eq!(part2(input), 19724);
}
