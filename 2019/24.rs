#![allow(clippy::unusual_byte_groupings)]

use advent::prelude::*;

fn parse_input(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => 0,
                '#' => 1,
                c => panic!("unexpected character `{}`", c),
            })
        })
        .enumerate()
        .fold(0, |acc, (b, t)| acc | (t << b))
}

fn default_input() -> u32 {
    parse_input(include_str!("input/24.txt"))
}

#[derive(Debug, Clone, Copy)]
struct Masks {
    /// The adjacency mask to be applied to the level below.
    lower: u32,
    /// The adjacency mask to be applied to the current level.
    current: u32,
    /// The adjacency mask to be applied to the level above.
    upper: u32,
}

impl Masks {
    const fn new() -> Masks {
        Masks {
            lower: 0,
            current: 0,
            upper: 0,
        }
    }
}

/// Compile time generated adjacency masks.
///
/// Since each layer is represented as a bit map we can represent the adjacent
/// locations for each location as a bit mask. Additionally, we can represent
/// any adjacent locations in the upper and lower layers as a bit mask as well.
/// Thus, for every location on the current layer we can generate three bit
/// masks. This means we can simply apply the mask to the layer and count the
/// number of bits to see the number of adjacent bugs on that layer.
///
/// The calculations for the masks are explained using the following diagram.
///
///      |     |         |     |
///   0  |  1  |    2    |  3  |   4
///      |     |         |     |
/// -----+-----+---------+-----+-----
///      |     |         |     |
///   5  |  6  |    7    |  8  |   9
///      |     |         |     |
/// -----+-----+---------+-----+-----
///      |     |A|B|C|D|E|     |
///      |     |-+-+-+-+-|     |
///      |     |F|G|H|I|J|     |
///      |     |-+-+-+-+-|     |
///  10  | 11  |K|L|?|N|O|  13 |  14
///      |     |-+-+-+-+-|     |
///      |     |P|Q|R|S|T|     |
///      |     |-+-+-+-+-|     |
///      |     |U|V|W|X|Y|     |
/// -----+-----+---------+-----+-----
///      |     |         |     |
///  15  | 16  |    17   |  18 |  19
///      |     |         |     |
/// -----+-----+---------+-----+-----
///      |     |         |     |
///  20  | 21  |    22   |  23 |  24
///      |     |         |     |
///
const MASKS: [Masks; 25] = masks();

/// The edge locations are the only locations we need to consider adjacencies
/// between the layers. Here we construct a simple map between the outer layer
/// location and the inner layer locations that interact. In other words:
/// *  7 => ABCDE
/// * 11 => AFKPU
/// * 13 => EJOTY
/// * 17 => UVWXY
#[rustfmt::skip]
const EDGES: [(u32, u32); 4] = [
    (0b00000_00000_00000_00100_00000, 0b00000_00000_00000_00000_11111),
    (0b00000_00000_00010_00000_00000, 0b00001_00001_00001_00001_00001),
    (0b00000_00000_01000_00000_00000, 0b10000_10000_10000_10000_10000),
    (0b00000_00100_00000_00000_00000, 0b11111_00000_00000_00000_00000),
];

/// The center location.
const CENTER: u32 = 0b00000_00000_00100_00000_00000;

/// Calculates the adjacent points on the current layer.
const fn current_mask(bit: i32) -> u32 {
    const OFFSETS: [i32; 4] = [-5, -1, 1, 5];
    let mut mask = 0;
    let mut i = 0;
    while i < OFFSETS.len() {
        let off = OFFSETS[i];
        if (off != -1 || bit % 5 != 0) && (off != 1 || bit % 5 != 4) {
            if let b @ 0..=24 = bit + off {
                mask |= 1 << b;
            }
        }
        i += 1;
    }
    mask
}

/// Calculates the adjacent points on the lower layer i.e. looking deeper.
const fn lower_mask(pos: u32) -> u32 {
    let mut i = 0;
    while i < EDGES.len() {
        let (edge, side) = EDGES[i];
        if pos == edge {
            return side;
        }
        i += 1;
    }
    0
}

/// Calculates the adjacent points on the upper layer i.e. looking shallower.
const fn upper_mask(pos: u32) -> u32 {
    let mut mask = 0;
    let mut i = 0;
    while i < EDGES.len() {
        let (edge, side) = EDGES[i];
        if side & pos > 0 {
            mask |= edge;
        }
        i += 1;
    }
    mask
}

/// Constructs the adjacency bit masks for every location on the current layer.
const fn masks() -> [Masks; 25] {
    let mut masks = [Masks::new(); 25];
    let mut bit = 0;
    while bit < 25 {
        let pos = 1 << bit;
        masks[bit as usize] = Masks {
            lower: lower_mask(pos),
            current: current_mask(bit),
            upper: upper_mask(pos),
        };
        bit += 1;
    }
    masks
}

/// Grow each location on a single layer using the provided `bugs` function to
/// count the neighbouring bugs.
fn grow(layer: u32, bugs: impl Fn(usize) -> u32) -> u32 {
    (0..=24)
        .filter(|&bit| {
            let has_bug = layer & (1 << bit) > 0;
            let adj_bugs = bugs(bit as usize);
            matches!((has_bug, adj_bugs), (false, 1 | 2) | (true, 1))
        })
        .fold(0, |acc, bit| acc | (1 << bit))
}

/// Grow the recursive space.
///
/// This function will pad the space with empty layers if needed and for each
/// layer grow it using a `bugs` function that considers the upper and lower
/// layer.
fn grow_recursive(mut space: VecDeque<u32>) -> VecDeque<u32> {
    // Pad with empty levels if needed
    if let Some(1..) = space.front() {
        space.push_front(0);
    }
    if let Some(1..) = space.back() {
        space.push_back(0);
    }
    space
        .iter()
        .enumerate()
        .map(|(i, &layer)| {
            // Get the lower layer
            let lower = i
                .checked_sub(1)
                .and_then(|i| space.get(i).copied())
                .unwrap_or(0);
            // Get the upper layer
            let upper = space.get(i + 1).copied().unwrap_or(0);
            // Grow the layer
            let layer = grow(layer, |bit| {
                let masks = &MASKS[bit as usize];
                (lower & masks.lower).count_ones()
                    + (layer & masks.current).count_ones()
                    + (upper & masks.upper).count_ones()
            });
            // Make sure to always unset the center location, this is where the
            // recursion happens the `grow` function doesn't know about this.
            layer & !CENTER
        })
        .collect()
}

fn part1(mut layer: u32) -> u32 {
    let mut seen = HashSet::new();
    while seen.insert(layer) {
        layer = grow(layer, |bit| (layer & MASKS[bit].current).count_ones());
    }
    layer
}

fn part2(layer: u32, n: usize) -> u32 {
    let mut space = VecDeque::from([layer]);
    for _ in 0..n {
        space = grow_recursive(space);
    }
    space.into_iter().map(u32::count_ones).sum()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input(), 200));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
....#
#..#.
#..##
..#..
#....",
    );
    assert_eq!(part1(input), 2129920);
    assert_eq!(part2(input, 10), 99);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 28772955);
    assert_eq!(part2(input, 200), 2023);
}
