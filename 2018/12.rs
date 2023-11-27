use advent::prelude::*;

fn parse_input(input: &str) -> (BTreeSet<i32>, Vec<i32>) {
    let [initial, rules] = input.split("\n\n").next_array().unwrap();

    // Create a set containing the numbers of the pots with a plant.
    let pots: BTreeSet<i32> = initial
        .strip_prefix("initial state: ")
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').some(i as i32))
        .collect();

    // Parse each rule as a 5-bit number.
    let rules: Vec<i32> = rules
        .lines()
        .filter(|line| !line.ends_with('.'))
        .map(|line| {
            line.split(" => ")
                .next()
                .unwrap()
                .chars()
                .map(|c| (c == '#') as i32)
                .fold(0, |acc, p| acc * 2 + p)
        })
        .collect();

    (pots, rules)
}

fn default_input() -> (BTreeSet<i32>, Vec<i32>) {
    parse_input(include_str!("input/12.txt"))
}

fn next(pots: BTreeSet<i32>, rules: &[i32]) -> BTreeSet<i32> {
    let min = *pots.iter().next().unwrap();
    let max = *pots.iter().next_back().unwrap();
    let all = (min - 2)..=(max + 2);
    all.filter(|&i| {
        let w = [i - 2, i - 1, i, i + 1, i + 2]
            .into_iter()
            .map(|i| pots.contains(&i) as i32)
            .fold(0, |acc, b| acc * 2 + b);
        rules.contains(&w)
    })
    .collect()
}

fn part1((mut pots, rules): (BTreeSet<i32>, Vec<i32>)) -> i32 {
    for _ in 0..20 {
        pots = next(pots, &rules);
    }
    pots.iter().sum()
}

fn part2((mut pots, rules): (BTreeSet<i32>, Vec<i32>)) -> i64 {
    let mut gen = 0;
    let mut sum = 0;
    let mut diff = 0;
    loop {
        pots = next(pots, &rules);
        let s = pots.iter().sum::<i32>() as i64;
        let d = s - sum;
        if d == diff {
            break;
        }
        sum = s;
        diff = d;
        gen += 1;
    }
    (50_000_000_000 - gen) * diff + sum
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
    );
    assert_eq!(part1(input), 325);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1_733);
    assert_eq!(part2(input), 1_000_000_000_508);
}
