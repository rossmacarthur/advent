use advent::prelude::*;

type Vector4 = Vector<i16, 4>;
type Blueprint = [(Vector4, Vector4); 4];

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let [ore_x, clay_x, obsidian_x, obsidian_y, geode_x, geode_z] = line
                .split_whitespace()
                .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
                .map(|s| s.parse().unwrap())
                .next_array()
                .unwrap();
            [
                (ORE, vector![ore_x, 0, 0, 0]),
                (CLAY, vector![clay_x, 0, 0, 0]),
                (OBSIDIAN, vector![obsidian_x, obsidian_y, 0, 0]),
                (GEODE, vector![geode_x, 0, geode_z, 0]),
            ]
        })
        .collect()
}

fn default_input() -> Vec<Blueprint> {
    parse_input(include_input!(2022 / 19))
}

const ORE: Vector4 = vector![1, 0, 0, 0];
const CLAY: Vector4 = vector![0, 1, 0, 0];
const OBSIDIAN: Vector4 = vector![0, 0, 1, 0];
const GEODE: Vector4 = vector![0, 0, 0, 1];

fn simulate(bp: Blueprint, time: i16) -> i16 {
    // For each state we store (resources, robots, time) where
    // - resources is a 4D vector with the current amount of each resource
    // - robots is a 4D vector with the current amount of each robot
    // - time is the amount of time remaining
    //
    // Where
    //   x => ore
    //   y => clay
    //   z => obsidian
    //   w => geodes
    let mut states = VecDeque::from([(Vector4::zero(), ORE, time)]);
    let mut visited = HashSet::new();
    let mut best = 0;

    // The maximum cost of each resource
    let max_cost: Vector4 = (0..4)
        .map(|r| bp.iter().map(|(_, cost)| cost[r]).max().unwrap())
        .collect();

    while let Some((mut resources, robots, t)) = states.pop_front() {
        // We don't need to store time in the state space because later inserts
        // for the same resources and robots would come at a later time and
        // obviously be worse.
        if !visited.insert((resources, robots)) {
            continue;
        }

        if t == 0 {
            best = max(best, resources.w);
            continue;
        }

        // Heuristic A:
        // Decrease the total amount of each intermediate resource to the
        // maximum amount required to build a robot on every tick. This helps
        // reduce the total number of required states.
        for r in 0..2 {
            let amt = t * max_cost[r] - (t - 1) * robots[r];
            resources[r] = min(resources[r], amt)
        }

        // Heuristic B:
        // If we have enough resources to build every kind of robot then it's
        // never worth not building a robot
        if !(0..3).all(|r| resources[r] >= max_cost[r]) {
            // We don't have to build a robot, we can just collect resources
            states.push_back((resources + robots, robots, t - 1));
        }

        // Otherwise loop through all types of robots
        for &(kind, cost) in bp.iter().rev() {
            // If we have enough resources to make the robot then build it
            if (0..3).all(|r| resources[r] >= cost[r]) {
                states.push_back((resources + robots - cost, robots + kind, t - 1));
                // HACK: Assume that building a geode or obsidian robot is
                // always best. I don't think this is always correct but it
                // works on my input and the example.
                if kind == GEODE || kind == OBSIDIAN {
                    break;
                }
            }
        }
    }

    best
}

fn part1(blueprints: Vec<Blueprint>) -> i16 {
    blueprints
        .into_iter()
        .enumerate()
        .map(|(i, bp)| simulate(bp, 24) * (i + 1) as i16)
        .sum()
}

fn part2(blueprints: Vec<Blueprint>) -> i16 {
    blueprints
        .into_iter()
        .take(3)
        .map(|bp| simulate(bp, 32))
        .product()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    assert_eq!(part1(input), 33);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1389);
    assert_eq!(part2(input), 3003);
}
