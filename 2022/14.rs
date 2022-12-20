use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector2> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(str::parse)
                        .map(Result::unwrap)
                        .next_array()
                        .map(Vector2::from)
                        .unwrap()
                })
                .array_windows()
                .flat_map(|[p1, p2]| {
                    let d = (p2 - p1).map(i64::signum);
                    iter::successors(Some(p1), move |&p| (p != p2).some(p + d))
                })
        })
        .collect()
}

fn default_input() -> HashSet<Vector2> {
    parse_input(include_str!("input/14.txt"))
}

fn solve(mut cave: HashSet<Vector2>, part2: bool) -> usize {
    use image::{ImageBuffer, Rgb};

    let prefix = if part2 { "part2" } else { "part1" };

    let min_x = cave.iter().map(|v| v.x).min().unwrap();
    let min_y = cave.iter().map(|v| v.y).min().unwrap();
    let mut max_y = cave.iter().map(|v| v.y).max().unwrap();
    if part2 {
        max_y += 2;
    }

    let shift_x: i64 = 40;
    let shift_y: i64 = 3;

    // the padding around the image
    let pad = 20;
    // the size of each sand block
    let block = 5;

    let height = (max_y - min_y) as u32;
    let mut img = ImageBuffer::new(
        block * height + (2 * shift_y as u32) + 2 * pad,
        block * height + (2 * shift_y as u32) + 2 * pad,
    );

    // Draw a gray background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([0x1f, 0x1f, 0x1f]);
    }
    let draw_point = |img: &mut ImageBuffer<_, _>, p: Vector2, color: Rgb<u8>| {
        for dx in 0..block {
            for dy in 0..block {
                let x = block * (p.x - min_x + shift_x) as u32 + dx;
                let y = block * (p.y - min_y + shift_y) as u32 + dy;
                if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                    *pixel = color;
                } else {
                    // eprintln!("{p:?} out of bounds -> {x},{y}");
                }
            }
        }
    };

    // Draw the walls
    for p in &cave {
        draw_point(&mut img, *p, Rgb([0x7f, 0x7f, 0x7f]));
    }

    let mut flowing = vec![vector![500, 0]];
    let mut count = 0;
    let mut id = 0;

    let dir = std::path::PathBuf::from_iter([env!("CARGO_WORKSPACE_DIR"), "target", "visual"]);
    std::fs::create_dir_all(&dir).unwrap();

    while let Some(sand) = flowing.last().copied() {
        if (count < 1000 && count % 10 == 0)
            || (count < 2000 && count % 20 == 0)
            || (count >= 2000 && count % 40 == 0)
        {
            let mut img2 = img.clone();
            for f in &flowing {
                draw_point(&mut img2, *f, Rgb([0xb2, 0x70, 0x70]));
            }
            let p = dir.join(format!("{prefix}-{id:04}.png"));
            println!("{}", p.display());
            img2.save(p).unwrap();
            id += 1;
            // break;
        }

        // Check if there is air in any of the following directions
        match vectors![[0, 1], [-1, 1], [1, 1]]
            .into_iter()
            .map(|d| sand + d)
            .find_map(|p| (!cave.contains(&p)).some(p))
        {
            // There is space for the sand to flow, so update the sand position
            // and continue flowing
            Some(p) if p.y < max_y => flowing.push(p),
            // We've gone out of bounds, if part 1 then we're done
            Some(_) if !part2 => break,
            // Otherwise we can no longer flow, so we just insert sand at this
            // location
            _ => {
                draw_point(&mut img, sand, Rgb([0xb2, 0xa2, 0x70]));
                cave.insert(sand);
                flowing.pop();
                count += 1;
            }
        }
    }
    count
}

fn part1(cave: HashSet<Vector2>) -> usize {
    solve(cave, false)
}

fn part2(cave: HashSet<Vector2>) -> usize {
    solve(cave, true)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    assert_eq!(part1(input.clone()), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 774);
    assert_eq!(part2(input), 22499);
}
