// Advent of Code

//     [About][Events][Shop][Settings][Log Out]

// Ross MacArthur 40* var y=2023;

//     [Calendar][AoC++][Sponsors][Leaderboard][Stats]

// Our sponsors help make Advent of Code possible: CostPerform - Cracking codes
// + solving puzzles == fun == CostPerform --- Day 21: Step Counter ---

// You manage to catch the airship right as it's dropping someone else off on
// their all-expenses-paid trip to Desert Island! It even helpfully drops you
// off near the gardener and his massive farm.

// "You got the sand flowing again! Great work! Now we just need to wait until
// we have enough sand to filter the water for Snow Island and we'll have snow
// again in no time."

// While you wait, one of the Elves that works with the gardener heard how good
// you are at solving problems and would like your help. He needs to get his
// steps in for the day, and so he'd like to know which garden plots he can
// reach with exactly his remaining 64 steps.

// He gives you an up-to-date map (your puzzle input) of his starting position
// (S), garden plots (.), and rocks (#). For example:

// ........... .....###.#. .###.##..#. ..#.#...#.. ....#.#.... .##..S####.
// .##..#...#. .......##.. .##.#.####. .##..##.##. ...........

// The Elf starts at the starting position (S) which also counts as a garden
// plot. Then, he can take one step north, south, east, or west, but only onto
// tiles that are garden plots. This would allow him to reach any of the tiles
// marked O:

// ........... .....###.#. .###.##..#. ..#.#...#.. ....#O#.... .##.OS####.
// .##..#...#. .......##.. .##.#.####. .##..##.##. ...........

// Then, he takes a second step. Since at this point he could be at either tile
// marked O, his second step would allow him to reach any garden plot that is
// one step north, south, east, or west of any tile that he could have reached
// after the first step:

// ........... .....###.#. .###.##..#. ..#.#O..#.. ....#.#.... .##O.O####.
// .##.O#...#. .......##.. .##.#.####. .##..##.##. ...........

// After two steps, he could be at any of the tiles marked O above, including
// the starting position (either by going north-then-south or by going
// west-then-east).

// A single third step leads to even more possibilities:

// ........... .....###.#. .###.##..#. ..#.#.O.#.. ...O#O#.... .##.OS####.
// .##O.#...#. ....O..##.. .##.#.####. .##..##.##. ...........

// He will continue like this until his steps for the day have been exhausted.
// After a total of 6 steps, he could reach any of the garden plots marked O:

// ........... .....###.#. .###.##.O#. .O#O#O.O#.. O.O.#.#.O.. .##O.O####.
// .##.O#O..#. .O.O.O.##.. .##.#.####. .##O.##.##. ...........

// In this example, if the Elf's goal was to get exactly 6 more steps today, he
// could use them to reach any of 16 garden plots.

// However, the Elf actually needs to get 64 steps today, and the map he's
// handed you is much larger than the example map.

// Starting from the garden plot marked S on your map, how many garden plots
// could the Elf reach in exactly 64 steps?

// To begin, get your puzzle input.

// Answer:

// You can also [Shareon Twitter Mastodon] this puzzle.

use advent::prelude::*;

fn parse_input(input: &str) -> Map {
    let mut map: HashMap<_, _> = parse_map(input, |c| match c {
        '#' => Tile::Rock,
        '.' => Tile::GardenPlot,
        'S' => Tile::Start,
        _ => panic!("unexpected character `{}`", c),
    });

    let start = map
        .iter_mut()
        .find_map(|(p, t)| {
            if *t == Tile::Start {
                *t = Tile::GardenPlot;
                return Some(*p);
            }
            None
        })
        .unwrap();

    let width = map.keys().map(|p| p.x).max().unwrap() + 1;
    let height = map.keys().map(|p| p.y).max().unwrap() + 1;

    Map {
        map,
        width,
        height,
        start,
    }
}

fn default_input() -> Map {
    parse_input(include_input!(2023 / 21))
}

#[derive(Debug, Clone)]
struct Map {
    map: HashMap<Vector2, Tile>,
    width: i64,
    height: i64,
    start: Vector2,
}
impl Map {
    fn get(&self, p: &Vector2) -> Option<&Tile> {
        self.map.get(p)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

const CARDINALS: [Vector2; 4] = vectors!([0, 1], [-1, 0], [0, -1], [1, 0]);

fn solve(map: &Map, start: Vector2, n: i64) -> usize {
    let mut q = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();
    let mut count = 0;
    while let Some((p, steps)) = q.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        if (steps % 2) == (n % 2) {
            count += 1;
        };
        if steps == n {
            continue;
        }
        for d in CARDINALS {
            let next = p + d;
            let wrap = vector![next.x.rem_euclid(map.width), next.y.rem_euclid(map.height)];
            if let Some(Tile::GardenPlot) = map.get(&wrap) {
                q.push_back((next, steps + 1));
            }
        }
    }
    count
}

fn part1(map: Map, n: i64) -> usize {
    solve(&map, map.start, n)
}

fn part2(map: Map, n: i64) -> i64 {
    assert_eq!(map.width, map.height, "solution only works for square maps");
    assert_eq!(map.width % 2, 1, "solution only works for odd sized maps");

    for i in 0..3 {
        let x = map.width / 2 + i * map.width;
        let steps = solve(&map, map.start, x);
        println!("f({x}) = {steps}");
    }

    // we

    // println!("x: {}", x);
    todo!()
    // let start = input
    //     .iter()
    //     .find_map(|(&k, &t)| (t == Tile::Start).some(k))
    //     .unwrap();

    // let max_x = input.keys().map(|p| p.x).max().unwrap() + 1;
    // let max_y = input.keys().map(|p| p.y).max().unwrap() + 1;

    // let mut q = VecDeque::from([(start, 0)]);
    // // let mut visited = HashSet::new();
    // let mut counted = HashSet::new();

    // let mut grid_counts: HashMap<Vector2, i64> = HashMap::new();

    // loop {
    //     let mut count = 0;
    //     let mut max_steps = max_steps;
    //     while let Some((p, steps)) = q.pop_front() {
    //         // if !visited.insert(p) {
    //         //     continue;
    //         // }
    //         println!(
    //             "{:?}",
    //             grid_counts
    //                 .iter()
    //                 .sorted_by_key(|(&p, d)| p.l1_norm())
    //                 .map(|(_, c)| c)
    //                 .collect::<Vec<_>>()
    //         );
    //         // if steps % 2 == 0 {

    //         if steps % 2 && steps == max_steps {
    //             if counted.insert(p) {
    //                 *grid_counts
    //                     .entry(vector![p.x / max_x, p.y / max_y])
    //                     .or_insert(0) += 1;
    //                 // }
    //             }
    //             continue;
    //         }
    //         for d in CARDINALS {
    //             let next = p + d;
    //             let n = vector![next.x.rem_euclid(max_x), next.y.rem_euclid(max_y)];
    //             if let Some(Tile::GardenPlot) = input.get(&n) {
    //                 q.push_back((next, steps + 1));
    //             }
    //         }
    //     }
    // }

    // grid_counts.values().sum()
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i, 64))
        .part(|i| part2(i, 26501365))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    );
    assert_eq!(part1(input.clone(), 6), 16);
    assert_eq!(part1(input.clone(), 10), 50);
    assert_eq!(part1(input.clone(), 50), 1594);
    assert_eq!(part1(input.clone(), 100), 6536);
    assert_eq!(part1(input.clone(), 500), 167004);
    assert_eq!(part1(input.clone(), 1000), 668697);
    assert_eq!(part1(input, 5000), 16733044);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 64), 3722);
    assert_eq!(part2(input, 26501365), 2);
}
