use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2, i64)> {
    regex!(
        r"Sensor at x=(?P<sx>\-?\d+), y=(?P<sy>\-?\d+): closest beacon is at x=(?P<bx>\-?\d+), y=(?P<by>\-?\d+)"
    ).captures_iter(input)
        .map(|caps| {
            let sx: i64 = caps["sx"].parse().unwrap();
            let sy: i64 = caps["sy"].parse().unwrap();
            let bx: i64 = caps["bx"].parse().unwrap();
            let by: i64 = caps["by"].parse().unwrap();
            let sensor = vector![sx, sy];
            let beacon = vector![bx, by];
            // Precompute the distance since its needed everywhere
            let d = (sensor - beacon).l1_norm();
            (sensor, beacon, d)
        })
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2, i64)> {
    parse_input(include_str!("input/15.txt"))
}

/// Returns true if the given point is in range of any sensor.
fn in_range(data: &[(Vector2, Vector2, i64)], x: i64, y: i64) -> bool {
    let p = vector![x, y];
    data.iter()
        .any(|&(sensor, _, d)| (sensor - p).l1_norm() <= d)
}

/// Returns the intersection point of two lines.
fn intersects((m1, c1): (i64, i64), (m2, c2): (i64, i64)) -> [i64; 2] {
    // Some quick primary school maths to find the intersection of the two
    // lines:
    //   y = m1 * x + c1
    //   y = m2 * x + c2
    //
    // Solve
    // =>  m1*x + c1 = m2*x + c2
    // =>  (m1 - m2)*x = c2 - c1
    // =>  x = (c2 - c1) / (m1 - m2)
    //
    let x = (c2 - c1) / (m1 - m2);
    let y = m1 * x + c1;
    [x, y]
}

fn lines(data: &[(Vector2, Vector2, i64)]) -> (Vec<i64>, Vec<i64>) {
    // For m = 1 which is line going from bottom left to top right
    let mut up = Vec::new();
    // For m = -1 which is a line going from top left to bottom right
    let mut down = Vec::new();

    for &(sensor, _, d) in data {
        // Manhattan distance plus one
        let d = d + 1;

        // Now find the line equation for each of the four diamond edges...
        for [m, side] in [[-1, -1], [-1, 1], [1, -1], [1, 1]] {
            // Shift the y point of the sensor such that it becomes the top or
            // bottom corner of the diamond.
            let [x, y] = [sensor.x, sensor.y + (d * side)];

            // Then use this point to solve for c in the straight line equation:
            //     y = mx + c
            // =>  c = y - mx
            let c = y - m * x;

            match m {
                1 => up.push(c),
                -1 => down.push(c),
                _ => unreachable!(),
            }
        }
    }

    (up, down)
}

fn part1(data: Vec<(Vector2, Vector2, i64)>, row: i64) -> usize {
    let mut count = 0;
    for x in (-5 * row)..(5 * row) {
        if in_range(&data, x, row) {
            count += 1;
        }
    }
    let objs: HashSet<_> = data.into_iter().flat_map(|(s, b, _)| [s, b]).collect();
    for obj in &objs {
        if obj.y == row {
            count -= 1;
        }
    }
    count
}

// In order for there to only be one single point not covered by any of the
// sensors it would need to be at the edge of four separate sensor areas,
// otherwise there could be multiple solutions. (Nit: I think technically if it
// lay on the edge of the bounding box this logic won't work, but lets assume it
// isn't.) Using this logic we can do the following.
//
// - Take each sensor's diamond area increased by 1 and find the edges of this
//   shape as straight lines.
// - Take the lines from all the diamonds and find the all the intersection
//   points.
// - One of these must be the distress beacon!
//
// To implement this we will represent each of these lines as m and c as in y =
// mx + c. Since all of the lines only have one of two m values that and lines
// with the same m value won't intersect we store to lists for each of the c
// values.
fn part2(data: Vec<(Vector2, Vector2, i64)>, max: i64) -> i64 {
    let (up, down) = lines(&data);

    let bounds = 0..=max;
    for (&c1, &c2) in iproduct!(&up, &down) {
        // Find the point that these two lines intersect
        let [x, y] = intersects((1, c1), (-1, c2));

        // If it is within bounds and not in range of any sensor then we have
        // found the distress beacon!
        if bounds.contains(&x) && bounds.contains(&y) && !in_range(&data, x, y) {
            return x * 4_000_000 + y;
        }
    }

    panic!("distress beacon not found")
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(|i| part1(i, 2_000_000));
    run.part(|i| part2(i, 4_000_000));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    );
    assert_eq!(part1(input.clone(), 10), 26);
    assert_eq!(part2(input, 20), 56000011);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 2_000_000), 5073496);
    assert_eq!(part2(input, 4_000_000), 13081194638237);
}
