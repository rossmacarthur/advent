use std::ops::Range;

use advent::prelude::*;

fn parse_claim(input: &str) -> Claim {
    let re = regex!(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<dx>\d+)x(?P<dy>\d+)");
    let caps = re.captures(input).unwrap();
    let id = caps["id"].parse().unwrap();
    let x = caps["x"].parse().unwrap();
    let y = caps["y"].parse().unwrap();
    let dx = caps["dx"].parse().unwrap();
    let dy = caps["dy"].parse().unwrap();
    let loc = vector![x, y];
    let size = vector![dx, dy];
    Claim { id, loc, size }
}

fn parse_input(input: &str) -> Vec<Claim> {
    input.lines().map(parse_claim).collect()
}

fn default_input() -> Vec<Claim> {
    parse_input(include_str!("input/03.txt"))
}

#[derive(Debug, Copy, Clone)]
struct Claim {
    id: i64,
    loc: Vector2,
    size: Vector2,
}

fn xs(claim: &Claim) -> Range<i64> {
    claim.loc.x..(claim.loc.x + claim.size.x)
}

fn ys(claim: &Claim) -> Range<i64> {
    claim.loc.y..(claim.loc.y + claim.size.y)
}

fn squares(claim: &Claim) -> impl Iterator<Item = Vector2> + '_ {
    xs(claim).flat_map(|x| ys(claim).map(move |y| vector![x, y]))
}

fn fabric(claims: &[Claim]) -> HashMap<Vector2, usize> {
    let mut f: HashMap<Vector2, usize> = HashMap::new();
    for claim in claims {
        for sq in squares(claim) {
            *f.entry(sq).or_default() += 1;
        }
    }
    f
}

fn part1(claims: Vec<Claim>) -> usize {
    fabric(&claims).values().filter(|&&n| n > 1).count()
}

fn part2(claims: Vec<Claim>) -> i64 {
    let f = fabric(&claims);
    claims
        .iter()
        .find_map(|claim| squares(claim).all(|sq| f[&sq] == 1).then(|| claim.id))
        .unwrap()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 117948);
    assert_eq!(part2(input), 567);
}
