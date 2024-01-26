mod derp;
use std::ops::RangeInclusive;

use advent::prelude::*;

fn parse_vec3(input: &str) -> Vector3 {
    input
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            (parse_vec3(pos), parse_vec3(vel))
        })
        .collect()
}

fn default_input() -> Vec<Stone> {
    parse_input(include_input!(2023 / 24))
}

type Vector2 = Vector<f64, 2>;
type Vector3 = Vector<f64, 3>;
type Stone = (Vector3, Vector3);

/// Returns true if the given float is basically zero.
fn is_zero(f: f64) -> bool {
    f.abs() < f64::EPSILON
}

/// Returns the dot product of two vectors.
fn dot(a: Vector3, b: Vector3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Returns the cross product of two vectors.
fn cross(a: Vector3, b: Vector3) -> Vector3 {
    vector![
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    ]
}

/// Returns true if the given vectors are linearly independent.
fn linearly_independent(a: Vector3, b: Vector3) -> bool {
    cross(a, b).iter().any(|&x| !is_zero(x))
}

/// Given a hailstone at t=0 and its velocity, return the slope and y intercept
/// of the associated linear equation.
fn slope_intercept((p, v): Stone) -> (f64, f64) {
    let m = v.y / v.x;
    let c = p.y - m * p.x;
    (m, c)
}

// Calculate time to reach the intersection for each hailstone
fn intersection_time(p: Vector3, v: Vector3, x: f64, y: f64) -> Option<f64> {
    if !is_zero(v.x) {
        Some((x - p.x) / v.x)
    } else if !is_zero(v.y) {
        Some((y - p.y) / v.y)
    } else {
        None
    }
}

/// Returns the intersection point of two hailstones, if it will happen in the
/// future.
fn intersects((p1, v1): Stone, (p2, v2): Stone) -> Option<Vector2> {
    // Get the linear equations for each hailstone's trajectory
    let (m1, c1) = slope_intercept((p1, v1));
    let (m2, c2) = slope_intercept((p2, v2));

    if is_zero(m1 - m2) {
        // Hailstones are parallel
        return None;
    }

    // Calculate the intersection point of the two linear equations
    let x = (c2 - c1) / (m1 - m2);
    let y = m1 * x + c1;

    // Calculate time to reach the intersection for each hailstone
    let t1 = intersection_time(p1, v1, x, y)?;
    let t2 = intersection_time(p2, v2, x, y)?;

    // Only return the intersection if it will happen in the future
    (t1 > 0. && t2 > 0.).some(vector![x, y])
}

fn find_plane((p1, v1): Stone, (p2, v2): Stone) -> (Vector3, f64) {
    let dp = p1 - p2;
    let dv = v1 - v2;
    (cross(v1, v2), dot(dp, dv))
}

fn find_rock(h1: Stone, h2: Stone, h3: Stone) -> (Vector3, f64) {
    let (p1, v1) = h1;
    let (p2, v2) = h2;
    let (p3, v3) = h3;

    let (a, A) = find_plane(h1, h2);
    let (b, B) = find_plane(h1, h3);
    let (c, C) = find_plane(h2, h3);

    println!("a = {:?}\nb = {:?}\nc = {:?}", a, b, c);

    // w is the intersection of the three planes
    let w = cross(b, c) * A + cross(c, a) * B + cross(a, b) * C;
    // t is the time to reach the intersection
    let t = dot(a, cross(b, c));

    let w = (w / t).map(f64::round);

    let w1 = h1.1 - w;
    let w2 = h2.1 - w;
    let ww = cross(w1, w2);

    let E = dot(ww, cross(p2, w2));
    let F = dot(ww, cross(p1, w1));
    let G = dot(p1, ww);
    let S = dot(ww, ww);

    let rock = w1 * E + w2 * -F + ww * G;
    (rock, S)
}

fn part1(hailstones: Vec<Stone>, test_area: RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .array_combinations()
        .filter_map(|[&h1, &h2]| intersects(h1, h2))
        .filter(|&p| test_area.contains(&p.x) && test_area.contains(&p.y))
        .count()
}

fn iter_velocities() -> impl Iterator<Item = [f64; 2]> {
    (0..)
        .flat_map(|d| (0..=d).map(move |x| (x, d - x)))
        .flat_map(|(x, y)| cartesian_product!([-x, x], [-y, y]))
        .map(|(x, y)| [x as f64, y as f64])
}

fn try_velocity(hailstones: &[Stone], [dx, dy]: [f64; 2]) -> bool {
    let adjust = move |(p, v)| (p, v - vector![dx, dy, 0.0]);

    // Adjust the velocity of the first hailstone
    let h = adjust(hailstones[0]);

    // Compare the adjusted hailstone with every other adjusted
    // hailstone. If they all intersect then we've got a candidate for
    // the rock's velocity.
    let intersections: Vec<_> = hailstones[1..]
        .iter()
        .filter_map(|&h2| intersects(h, adjust(h2)))
        .collect();

    if intersections.len() + 1 != hailstones.len() {
        return false;
    }

    let (ix, iy) = (intersections[0].x, intersections[0].y);

    println!("{dx},{dy}");

    // if (dx, dy) != (131., -259.) {
    //     return false;
    // }

    let (min_x, max_x) = intersections
        .iter()
        .map(|i| i.x)
        .fold((ix, ix), |(min, max), x| (min.min(x), max.max(x)));

    let (min_y, max_y) = intersections
        .iter()
        .map(|i| i.y)
        .fold((iy, iy), |(min, may), y| (min.min(y), may.max(y)));

    println!("  x: {}..{} ({})", min_x, max_x, max_x - min_x);
    println!("  y: {}..{} ({})", min_y, max_y, max_y - min_y);

    // // Now try and find the z
    // if intersections
    //     .iter()
    //     .arrays()
    //     .any(|[i, j]| !is_zero(i.x - j.x) || !is_zero(i.y - j.y))
    // {
    //     return false;
    // }
    false
    // true
}

fn part2(hailstones: Vec<Stone>) -> i64 {
    // We can reframe the problem of finding the rock's position and velocity
    // to instead consider the rocks velocity as zero and the hailstones as all
    // having the same adjusted velocity.
    //
    // Let's just brute force a velocity adjustment. If the hailstones at that
    // velocity all intersect then that position is the rock's position.

    derp::main().unwrap();
    todo!();

    let n = hailstones.len();

    let h1 @ (p1, v1) = hailstones[0];
    let (i, h2 @ (p2, v2)) = hailstones
        .iter()
        .copied()
        .enumerate()
        .skip(1)
        .find(|&(i, (_, v2))| linearly_independent(v1, v2))
        .unwrap();
    let h3 @ (p3, v3) = hailstones
        .iter()
        .copied()
        .find(|&(_, v3)| linearly_independent(v1, v3) && linearly_independent(v2, v3))
        .unwrap();

    println!("h1 = {h1:?}\nh2 = {h2:?}\nh3 = {h3:?}");

    let (rock, s) = find_rock(h1, h2, h3);
    (rock.into_iter().sum::<f64>() / s) as i64
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i, 200e12..=400e12))
        .part(part2)
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
    );
    assert_eq!(part1(input, 7.0..=27.0), 2);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 200e12..=400e12), 16812);
    assert_eq!(part2(input), 880547248556435);
}
