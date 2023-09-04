use std::collections::hash_map::Entry;

use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, char> {
    parse_map(input, |c| c)
}

fn default_input() -> HashMap<Vector2, char> {
    parse_input(include_str!("input/13.txt"))
}

enum Track {
    // `|`
    Vertical,
    // `-`
    Horizontal,
    // `/`
    CurveUp,
    // `\`
    CurveDown,
    // `+`
    Intersection,
}

enum Turn {
    Left,
    Straight,
    Right,
}

fn solve(tiles: HashMap<Vector2, char>, first: bool) -> Vector2 {
    // Construct a clean map without any carts.
    let tracks: HashMap<_, _> = tiles
        .iter()
        .filter_map(|(&p, &c)| {
            let track = match c {
                '|' | '^' | 'v' => Track::Vertical,
                '-' | '<' | '>' => Track::Horizontal,
                '/' => Track::CurveUp,
                '\\' => Track::CurveDown,
                '+' => Track::Intersection,
                _ => return None,
            };
            Some((p, track))
        })
        .collect();

    // Store each cart's position, direction, and turn state.
    let mut carts: HashMap<_, _> = tiles
        .iter()
        .filter_map(|(&p, &c)| {
            let v = match c {
                '^' => vector![0, -1],
                'v' => vector![0, 1],
                '<' => vector![-1, 0],
                '>' => vector![1, 0],
                _ => return None,
            };
            Some((p, (v, Turn::Left)))
        })
        .collect();

    while carts.len() != 1 {
        for p in carts.keys().copied().sorted_by_key(|p| (p.y, p.x)) {
            let (d, t) = match carts.remove(&p) {
                Some(state) => state,
                None => {
                    // This can happen if this cart crashed on a previous
                    // iteration of the loop. Simply continue to the next cart.
                    continue;
                }
            };

            // Calculate the next position for the cart.
            let p = p + d;

            // Calculate the next direction and turn state for the cart.
            let (d, t) = match tracks[&p] {
                Track::Vertical | Track::Horizontal => (d, t),
                Track::CurveUp => (vector![-d.y, -d.x], t),
                Track::CurveDown => (vector![d.y, d.x], t),
                Track::Intersection => {
                    let d = match t {
                        Turn::Left => vector![d.y, -d.x],
                        Turn::Straight => d,
                        Turn::Right => vector![-d.y, d.x],
                    };
                    let t = match t {
                        Turn::Left => Turn::Straight,
                        Turn::Straight => Turn::Right,
                        Turn::Right => Turn::Left,
                    };
                    (d, t)
                }
            };

            // Add the cart back to the map of carts, if another cart is already
            // in this location then remove the existing cart.
            match carts.entry(p) {
                Entry::Vacant(e) => {
                    e.insert((d, t));
                }
                Entry::Occupied(e) => {
                    e.remove();
                    if first {
                        return p;
                    }
                }
            }
        }
    }
    carts.keys().next().copied().unwrap()
}

fn part1(tiles: HashMap<Vector2, char>) -> Vector2 {
    solve(tiles, true)
}

fn part2(tiles: HashMap<Vector2, char>) -> Vector2 {
    solve(tiles, false)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
",
    );
    assert_eq!(part1(input), vector![7, 3]);
}

#[test]
fn example2() {
    let input = parse_input(
        r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
",
    );
    assert_eq!(part2(input), vector![6, 4]);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), vector![118, 66]);
    assert_eq!(part2(input), vector![70, 129]);
}
