use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, char> {
    parse_map(input, |c| c)
}

fn default_input() -> HashMap<Vector2, char> {
    parse_input(include_input!(2024 / 12))
}

const N: Vector2 = vector![0, -1];
const E: Vector2 = vector![1, 0];
const S: Vector2 = vector![0, 1];
const W: Vector2 = vector![-1, 0];
const CARDINALS: [Vector2; 4] = [N, E, S, W];
const CORNERS: [[Vector2; 2]; 4] = [[N, E], [E, S], [S, W], [W, N]];

/// Returns the price of the region at `p`
fn get_region_price<const P: bool>(
    map: &HashMap<Vector2, char>,
    p: Vector2,
) -> (HashSet<Vector2>, usize) {
    let plant = Some(map[&p]); // the current plant
    let mut plots = HashSet::from_iter([p]); // the plots in the region
    let mut factor = 0; // perimiter for !P, number of sides for P

    let mut q = VecDeque::from([p]);
    let mut visited = HashSet::new();
    while let Some(p) = q.pop_front() {
        if !visited.insert(p) {
            continue;
        }

        if P {
            for [da, db] in CORNERS {
                // Variable naming is as follows:
                //
                //   ac
                //   pb
                //
                // Exterior corners can be identified by checking for:
                //
                //   OO
                //   XO
                //
                // Interior corners can be identified by checking for:
                //
                //   XO
                //   XX
                //
                // Where X is the plant and O is not the plant.
                //
                let a = map.get(&(p + da)).copied();
                let b = map.get(&(p + db)).copied();
                let c = map.get(&(p + da + db)).copied();
                if (a != plant && b != plant) || (a == plant && b == plant && c != plant) {
                    factor += 1;
                }
            }
        }

        for d in CARDINALS {
            let next = p + d;
            if map.get(&next).copied() == plant {
                plots.insert(next);
                q.push_back(next);
            } else if !P {
                factor += 1;
            }
        }
    }

    let price = plots.len() * factor;

    (plots, price)
}

fn get_price<const P: bool>(map: HashMap<Vector2, char>) -> usize {
    let mut total = 0;
    let mut visited = HashSet::new();
    for &p in map.keys() {
        if visited.contains(&p) {
            continue;
        }
        let (region, price) = get_region_price::<P>(&map, p);
        visited.extend(region.clone());
        total += price;
    }
    total
}

fn part1(map: HashMap<Vector2, char>) -> usize {
    get_price::<false>(map)
}

fn part2(map: HashMap<Vector2, char>) -> usize {
    get_price::<true>(map)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
AAAA
BBCD
BBCC
EEEC",
    );
    assert_eq!(part1(input.clone()), 140);
    assert_eq!(part2(input), 80);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    );
    assert_eq!(part1(input.clone()), 772);
    assert_eq!(part2(input), 436);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    );
    assert_eq!(part1(input.clone()), 1930);
    assert_eq!(part2(input), 1206);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1477762);
    assert_eq!(part2(input), 923480);
}
