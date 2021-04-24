use itertools::Itertools;
use math::lcm;
use regex_macro::regex;
use vectrix::{vector, Vector3};

type Vector = Vector3<i64>;

fn parse_input(input: &str) -> Vec<Moon> {
    regex!(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")
        .captures_iter(input)
        .map(|caps| {
            let pos = vector![
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap()
            ];
            let vel = Vector::zero();
            Moon { pos, vel }
        })
        .collect()
}

fn default_input() -> Vec<Moon> {
    parse_input(
        "<x=-1, y=-4, z=0>
        <x=4, y=7, z=-1>
        <x=-14, y=-10, z=9>
        <x=1, y=2, z=17>",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    fn energy(&self) -> i64 {
        self.pos.l1_norm() * self.vel.l1_norm()
    }
}

/// Simulate a set of moons moving, applies acceleration and velocity.
fn simulate(moons: &mut [Moon]) {
    for (m, n) in (0..moons.len()).tuple_combinations() {
        let (lower, upper) = moons.split_at_mut(n);
        let m = &mut lower[m];
        let n = &mut upper[0];
        for d in 0..3 {
            m.vel[d] += (n.pos[d] - m.pos[d]).signum();
            n.vel[d] += (m.pos[d] - n.pos[d]).signum();
        }
    }
    for moon in moons.iter_mut() {
        moon.pos += moon.vel;
    }
}

/// Simulate a set of moons on the given axis only.
fn simulate_axis(moons: &mut [Moon], d: usize) {
    for (m, n) in (0..moons.len()).tuple_combinations() {
        let (lower, upper) = moons.split_at_mut(n);
        let m = &mut lower[m];
        let n = &mut upper[0];
        m.vel[d] += (n.pos[d] - m.pos[d]).signum();
        n.vel[d] += (m.pos[d] - n.pos[d]).signum();
    }
    for moon in moons.iter_mut() {
        moon.pos[d] += moon.vel[d];
    }
}

/// Simulate a set of moons on the given axis only until the axis repeats.
fn simulate_axis_until_repeat(moons: &mut [Moon], d: usize) -> i64 {
    let start = moons.to_vec();
    for n in 1.. {
        simulate_axis(moons, d);
        if moons == start {
            return n;
        }
    }
    unreachable!()
}

fn part1(mut moons: Vec<Moon>, n: usize) -> i64 {
    for _ in 0..n {
        simulate(&mut moons);
    }
    moons.iter().map(Moon::energy).sum()
}

fn part2(mut moons: Vec<Moon>) -> i64 {
    let x = simulate_axis_until_repeat(&mut moons, 0);
    let y = simulate_axis_until_repeat(&mut moons, 1);
    let z = simulate_axis_until_repeat(&mut moons, 2);
    lcm(x, lcm(y, z))
}

fn main() {
    let mut run = advent::start();
    let input = run.time("Parse input", default_input());
    run.result("Part 1", part1(input.clone(), 1000));
    run.result("Part 2", part2(input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "<x=-1, y=0, z=2>
         <x=2, y=-10, z=-7>
         <x=4, y=-8, z=8>
         <x=3, y=5, z=-1>",
    );
    assert_eq!(part1(input.clone(), 10), 179);
    assert_eq!(part2(input), 2772);
}

#[test]
fn example2() {
    let input = parse_input(
        "<x=-8, y=-10, z=0>
         <x=5, y=5, z=10>
         <x=2, y=-7, z=3>
         <x=9, y=-8, z=-3>",
    );
    assert_eq!(part1(input.clone(), 100), 1940);
    assert_eq!(part2(input), 4686774924);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 1000), 7988);
    assert_eq!(part2(input), 337721412394184);
}
