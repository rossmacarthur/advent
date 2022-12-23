use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector2> {
    parse_map_set(input)
}

fn default_input() -> HashSet<Vector2> {
    parse_input(include_str!("input/23.txt"))
}

const N: Vector2 = vector![0, -1];
const S: Vector2 = vector![0, 1];
const E: Vector2 = vector![1, 0];
const W: Vector2 = vector![-1, 0];
const NE: Vector2 = vector![1, -1];
const NW: Vector2 = vector![-1, -1];
const SE: Vector2 = vector![1, 1];
const SW: Vector2 = vector![-1, 1];

fn solve<I>(mut grove: HashSet<Vector2>, rounds: I) -> i64
where
    I: Iterator<Item = usize>,
{
    // All the possible directions from an elf
    let all = [N, S, E, W, NE, NW, SE, SW];

    // Each set of directions to consider
    let consider = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

    // All the proposed moves by elves
    let mut proposed: HashMap<Vector2, Vec<Vector2>> = HashMap::new();

    use image::{ImageBuffer, Rgb};

    let min_x = -14;
    let max_x = 122;
    let min_y = -13;
    let max_y = 121;

    let pad: u32 = 2;
    let blk: u32 = 4;

    let height = (max_y - min_y) as u32;
    let width = (max_x - min_x) as u32;

    let mut img = ImageBuffer::new(blk * (width + pad), blk * (height + pad));
    for pixel in img.pixels_mut() {
        *pixel = Rgb([0x1f, 0x1f, 0x1f]);
    }

    let draw_point = |img: &mut ImageBuffer<_, _>, p: Vector2, color: Rgb<u8>| {
        for dx in 1..blk {
            for dy in 1..blk {
                let x = blk * ((p.x - min_x) as u32 + pad / 2) + dx;
                let y = blk * ((p.y - min_y) as u32 + pad / 2) + dy;
                if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                    *pixel = color;
                } else {
                    panic!("{p:?} out of bounds -> {x},{y}");
                }
            }
        }
    };

    let dir = std::path::PathBuf::from_iter([env!("CARGO_WORKSPACE_DIR"), "target", "visual"]);
    std::fs::create_dir_all(&dir).unwrap();

    for round in rounds {
        for p in &grove {
            let is_open = |ds: &[_]| ds.iter().all(|d| !grove.contains(&(p + d)));

            if is_open(&all) {
                continue;
            }

            for &way in consider.iter().cycle().skip(round % 4).take(4) {
                if is_open(&way) {
                    proposed.entry(p + way[0]).or_default().push(*p);
                    break;
                }
            }
        }

        let mut moving = HashSet::new();
        for (p, srcs) in &proposed {
            if srcs.len() > 1 {
                continue;
            }
            moving.insert(srcs[0]);
        }

        {
            let mut img2 = img.clone();
            for p in &grove {
                if moving.contains(&p) {
                    // #8BC6FC
                    draw_point(&mut img2, *p, Rgb([0x8b, 0xc6, 0xfc]));
                } else {
                    draw_point(&mut img2, *p, Rgb([0xee, 0xee, 0xee]));
                }
            }
            let p = dir.join(format!("{round:04}.png"));
            println!("{}", p.display());
            img2.save(p).unwrap();
        }

        // Part 2, can no longer continue, return the round number
        if proposed.is_empty() {
            return (round + 1) as i64;
        }

        for (p, srcs) in proposed.drain() {
            if srcs.len() > 1 {
                continue;
            }
            grove.remove(&srcs[0]);
            grove.insert(p);
        }
    }

    // Part 1, return the rectangle size
    let min_x = grove.iter().map(|p| p.x).min().unwrap();
    let max_x = grove.iter().map(|p| p.x).max().unwrap();
    let min_y = grove.iter().map(|p| p.y).min().unwrap();
    let max_y = grove.iter().map(|p| p.y).max().unwrap();
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    area - grove.len() as i64
}

fn part1(grove: HashSet<Vector2>) -> i64 {
    // solve(grove, 0..10)
    0
}

fn part2(grove: HashSet<Vector2>) -> i64 {
    solve(grove, 0..)
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
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
    );
    assert_eq!(part1(input.clone()), 110);
    assert_eq!(part2(input), 20);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3849);
    assert_eq!(part2(input), 995);
}
