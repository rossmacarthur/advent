use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Region> {
    input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|region| {
            let (dims, counts) = region.split_once(':').unwrap();
            let [w, h] = dims
                .split('x')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_array();
            let counts = counts
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            Region { w, h, counts }
        })
        .collect()
}

fn default_input() -> Vec<Region> {
    parse_input(include_input!(2025 / 12))
}

#[derive(Clone)]
struct Region {
    w: usize,
    h: usize,
    counts: Vec<usize>,
}

fn part1(regions: Vec<Region>) -> usize {
    regions
        .into_iter()
        .filter(|region| {
            let shapes: usize = region.counts.iter().sum();
            (region.w / 3) * (region.h / 3) >= shapes
        })
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 521);
}
