use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let [lo, hi] = line
                .split('-')
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap();
            Range(lo, hi + 1)
        })
        .collect();
    let ingredients = ingredients
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    (ranges, ingredients)
}

fn default_input() -> (Vec<Range>, Vec<i64>) {
    parse_input(include_input!(2025 / 05))
}

#[derive(Debug, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    fn contains(self, v: i64) -> bool {
        let Self(lo, hi) = self;
        lo <= v && v < hi
    }

    fn extend(self, hi: i64) -> Self {
        let Self(lo, _) = self;
        Self(lo, hi)
    }

    fn len(self) -> i64 {
        let Self(lo, hi) = self;
        hi - lo
    }
}

fn part1((ranges, ingredients): (Vec<Range>, Vec<i64>)) -> usize {
    ingredients
        .into_iter()
        .filter(|&ingred| ranges.iter().any(|range| range.contains(ingred)))
        .count()
}

fn part2((ranges, _): (Vec<Range>, Vec<i64>)) -> i64 {
    ranges
        .into_iter()
        .sorted_by_key(|&Range(lo, _)| lo)
        .fold(Vec::new(), |mut acc, range @ Range(lo, hi)| {
            match acc.last_mut() {
                None => acc.push(range),
                Some(prev) => match (prev.contains(lo), prev.contains(hi)) {
                    (false, false) => acc.push(range),
                    (true, true) => (),
                    (true, false) => *prev = prev.extend(hi),
                    (false, true) => unreachable!("ranges are sorted by start"),
                },
            }
            acc
        })
        .into_iter()
        .map(Range::len)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    );
    assert_eq!(part1(input.clone()), 3);
    assert_eq!(part2(input), 14);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 638);
    assert_eq!(part2(input), 352946349407338);
}
