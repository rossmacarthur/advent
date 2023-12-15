use advent::prelude::*;

type Path = Vec<(Vector2, i64)>;

fn parse_input(input: &str) -> [Path; 2] {
    input
        .lines()
        .map(|line| {
            regex!(r"(L|R|D|U)(\d+)")
                .captures_iter(line)
                .map(|caps| {
                    let length: i64 = caps[2].parse().unwrap();
                    let direction = match &caps[1] {
                        "L" => vector![-1, 0],
                        "R" => vector![1, 0],
                        "D" => vector![0, -1],
                        "U" => vector![0, 1],
                        _ => unreachable!(),
                    };
                    (direction, length)
                })
                .collect()
        })
        .collect_array()
}

fn default_input() -> [Path; 2] {
    parse_input(include_input!(2019 / 03))
}

fn distances(path: &[(Vector2, i64)]) -> HashMap<Vector2, i64> {
    let mut distances = HashMap::new();
    let mut position = Vector2::zero();
    let mut distance = 0;
    for &(direction, length) in path {
        for _ in 0..length {
            position += direction;
            distance += 1;
            distances.insert(position, distance);
        }
    }
    distances
}

fn keys(distances: &HashMap<Vector2, i64>) -> HashSet<Vector2> {
    distances.iter().map(|(k, _)| *k).collect()
}

fn part1([p1, p2]: [Path; 2]) -> i64 {
    keys(&distances(&p1))
        .intersection(&keys(&distances(&p2)))
        .map(Vector2::l1_norm)
        .min()
        .unwrap()
}

fn part2([p1, p2]: [Path; 2]) -> i64 {
    let distances1 = distances(&p1);
    let distances2 = distances(&p2);
    keys(&distances1)
        .intersection(&keys(&distances2))
        .map(|position| distances1[position] + distances2[position])
        .min()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
R8,U5,L5,D3
U7,R6,D4,L4",
    );
    assert_eq!(part1(input.clone()), 6);
    assert_eq!(part2(input), 30);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
    );
    assert_eq!(part1(input.clone()), 159);
    assert_eq!(part2(input), 610);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );
    assert_eq!(part1(input.clone()), 135);
    assert_eq!(part2(input), 410);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 248);
    assert_eq!(part2(input), 28580);
}
