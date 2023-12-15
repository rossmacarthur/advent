use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    let mut it = input.split("\n\n");

    let seeds = it
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let maps = it
        .map(|map| {
            let (_, ranges) = map.split_once("map:\n").unwrap();
            ranges
                .lines()
                .map(|r| {
                    let [dst, src, d] = r
                        .split_whitespace()
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect_array();
                    RangeMap {
                        src: Range(src, src + d),
                        dst: Range(dst, dst + d),
                    }
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn default_input() -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    parse_input(include_input!(2023 / 05))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeMap {
    src: Range,
    dst: Range,
}

/// A range of numbers (start, end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range(i64, i64);

impl Range {
    fn contains(self, v: i64) -> bool {
        v >= self.0 && v < self.1
    }
}

/// Applies the mapping to the range returning the mapped ranges or `None` if it
/// can not be mapped, in the case of `None` further source ranges will be]
/// checked.
fn apply_map(r: Range, RangeMap { src, dst }: RangeMap) -> Option<Either<[Range; 1], [Range; 2]>> {
    let dr = dst.0 - src.0;
    Some(match (src.contains(r.0), src.contains(r.1 - 1)) {
        // The source does not contain the range
        //
        //     src |------------|      r |------|
        //
        (false, false) => return None,
        // The entire range can fit in the source
        //
        //          r |------|
        //     src |------------|
        //
        (true, true) => Either::Left([Range(r.0 + dr, r.1 + dr)]),
        // The left part of the range overlaps source
        //
        //                r |------|
        //     src |------------|
        //
        (true, false) => Either::Right([Range(r.0 + dr, src.1 + dr), Range(src.1, r.1)]),
        // The right part of the range overlaps the source
        //
        //   r |------|
        //     src |------------|
        //
        (false, true) => Either::Right([Range(src.0 + dr, r.1 + dr), Range(r.0, src.0)]),
    })
}

fn solve(mut ranges: Vec<Range>, maps: Vec<Vec<RangeMap>>) -> i64 {
    for map in maps {
        ranges = ranges
            .into_iter()
            .flat_map(|r| {
                map.iter()
                    .find_map(|&m| apply_map(r, m))
                    .unwrap_or(Either::Left([r]))
                    .into_iter()
            })
            .collect();
    }
    ranges.into_iter().map(|r| r.0).min().unwrap()
}

fn part1((seeds, maps): (Vec<i64>, Vec<Vec<RangeMap>>)) -> i64 {
    let seeds = seeds.into_iter().map(|s| Range(s, s + 1)).collect();
    solve(seeds, maps)
}

fn part2((seeds, maps): (Vec<i64>, Vec<Vec<RangeMap>>)) -> i64 {
    let seeds = seeds
        .into_iter()
        .arrays()
        .map(|[s, ds]| Range(s, s + ds))
        .collect();
    solve(seeds, maps)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    );
    assert_eq!(part1(input.clone()), 35);
    assert_eq!(part2(input), 46);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 331445006);
    assert_eq!(part2(input), 6472060);
}
