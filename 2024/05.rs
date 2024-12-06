use advent::prelude::*;

fn parse_input(input: &str) -> (HashSet<[i64; 2]>, Vec<Vec<i64>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<[i64; 2]> = rules
        .lines()
        .map(|line| {
            line.split("|")
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap()
        })
        .collect();

    let updates: Vec<Vec<i64>> = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    (rules, updates)
}

fn default_input() -> (HashSet<[i64; 2]>, Vec<Vec<i64>>) {
    parse_input(include_input!(2024 / 05))
}

fn solve(rules: HashSet<[i64; 2]>, updates: Vec<Vec<i64>>, part2: bool) -> i64 {
    updates
        .into_iter()
        .filter_map(|update| {
            let sorted = {
                let mut u = update.clone();
                u.sort_by(|&a, &b| {
                    if rules.contains(&[a, b]) {
                        Ordering::Less
                    } else if rules.contains(&[b, a]) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                u
            };
            ((update == sorted) ^ part2).some_with(|| sorted[sorted.len() / 2])
        })
        .sum()
}

fn part1((rules, updates): (HashSet<[i64; 2]>, Vec<Vec<i64>>)) -> i64 {
    solve(rules, updates, false)
}

fn part2((rules, updates): (HashSet<[i64; 2]>, Vec<Vec<i64>>)) -> i64 {
    solve(rules, updates, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part1(input.clone()), 143);
    assert_eq!(part2(input), 123);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5268);
    assert_eq!(part2(input), 5799);
}
