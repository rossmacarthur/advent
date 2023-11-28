use advent::prelude::*;

fn parse_input(input: &str) -> HashSet<Vector3> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn default_input() -> HashSet<Vector3> {
    parse_input(include_input!(2022 / 18))
}

const DIRECTIONS: &[Vector3] = &vectors!(
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1]
);

fn part1(cubes: HashSet<Vector3>) -> usize {
    cubes
        .iter()
        .flat_map(|&c| DIRECTIONS.iter().map(move |d| c + d))
        .filter(|p| !cubes.contains(p))
        .count()
}

fn part2(cubes: HashSet<Vector3>) -> usize {
    let min_x = cubes.iter().map(|v| v.x).min().unwrap() - 10;
    let max_x = cubes.iter().map(|v| v.x).max().unwrap() + 10;
    let min_y = cubes.iter().map(|v| v.y).min().unwrap() - 10;
    let max_y = cubes.iter().map(|v| v.y).max().unwrap() + 10;
    let min_z = cubes.iter().map(|v| v.z).min().unwrap() - 10;
    let max_z = cubes.iter().map(|v| v.z).max().unwrap() + 10;

    // Start the floodfill at the 8 corners of the bounding shape
    let mut q: VecDeque<_> = iproduct!([min_x, max_x], [min_y, max_y], [min_z, max_z])
        .map(Vector3::from)
        .collect();

    // This stores which points the steam can reach
    let mut steam = HashSet::new();
    while let Some(p) = q.pop_front() {
        if !steam.insert(p) {
            continue;
        }
        for d in DIRECTIONS {
            let n = p + d;
            if (min_x..max_x).contains(&n.x)
                && (min_y..max_y).contains(&n.y)
                && (min_z..max_z).contains(&n.z)
                && !cubes.contains(&n)
            {
                q.push_back(n);
            }
        }
    }

    // Sum all sides that the steam can reach, these must be the exterior sides
    // of the shape
    cubes
        .iter()
        .flat_map(|&c| DIRECTIONS.iter().map(move |d| c + d))
        .filter(|p| steam.contains(p))
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    assert_eq!(part1(input.clone()), 64);
    assert_eq!(part2(input), 58);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4504);
    assert_eq!(part2(input), 2556);
}
