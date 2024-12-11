use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, u32> {
    parse_map(input, |c| c.to_digit(10).unwrap())
}

fn default_input() -> HashMap<Vector2, u32> {
    parse_input(include_input!(2024 / 10))
}

const CARDINALS: [Vector2; 4] = vectors!([0, -1], [0, 1], [-1, 0], [1, 0]);

fn part1(map: HashMap<Vector2, u32>) -> usize {
    let mut q: Vec<_> = map
        .iter()
        .filter_map(|(&p, &h)| (h == 0).some((p, p, 0)))
        .collect();

    let mut trailheads = HashSet::new();
    let mut visited = HashSet::new();
    while let Some((start, p, height)) = q.pop() {
        if !visited.insert((start, p)) {
            continue;
        }
        for d in CARDINALS {
            let next = p + d;
            if let Some(&next_h) = map.get(&next) {
                if next_h == height + 1 {
                    if next_h == 9 {
                        trailheads.insert((start, next));
                    } else {
                        q.push((start, next, height + 1));
                    }
                }
            }
        }
    }

    trailheads.len()
}

fn part2(map: HashMap<Vector2, u32>) -> i64 {
    let mut q: Vec<_> = map
        .iter()
        .filter_map(|(&p, &h)| (h == 0).some((p, 0)))
        .collect();

    let mut rating = 0;

    while let Some((p, h)) = q.pop() {
        for d in CARDINALS {
            let np = p + d;
            if let Some(&nh) = map.get(&np) {
                if nh == h + 1 {
                    if nh == 9 {
                        rating += 1;
                    } else {
                        q.push((np, h + 1));
                    }
                }
            }
        }
    }

    rating
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    assert_eq!(part1(input.clone()), 36);
    assert_eq!(part2(input), 81);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 566);
    assert_eq!(part2(input), 1324);
}

// hi 1324
