use advent::prelude::*;

type Rules = HashMap<[char; 2], char>;

fn parse_input(input: &str) -> (&str, Rules) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let pair = left.chars().next_array().unwrap();
            let insert = right.chars().next().unwrap();
            (pair, insert)
        })
        .collect();

    (template, rules)
}

fn default_input() -> (&'static str, Rules) {
    parse_input(include_str!("input/14.txt"))
}

fn solve(template: &str, rules: Rules, steps: usize) -> usize {
    // Keep track of the count for each letter.
    let mut letters = template.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    // Keep track of the count for each pair of letters.
    let mut pairs = template
        .chars()
        .array_windows()
        .fold(HashMap::new(), |mut acc, pair| {
            *acc.entry(pair).or_insert(0) += 1;
            acc
        });

    for _ in 0..steps {
        // For every pair if there is a rule that applies to that pair then
        // split the pair AB into AC and CB keeping the count for both of them.
        pairs = pairs
            .into_iter()
            .flat_map(|([a, b], n)| {
                match rules.get(&[a, b]) {
                    Some(&i) => {
                        // Keep track of the letters here so we don't have to
                        // try and calculate it later.
                        *letters.entry(i).or_default() += n;
                        Either::Left([([a, i], n), ([i, b], n)])
                    }
                    None => Either::Right([([a, b], n)]),
                }
                .into_iter()
            })
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                *acc.entry(pair).or_default() += count;
                acc
            });
    }

    let min = letters.values().min().unwrap();
    let max = letters.values().max().unwrap();
    max - min
}

fn part1((template, rules): (&str, Rules)) -> usize {
    solve(template, rules, 10)
}

fn part2((template, rules): (&str, Rules)) -> usize {
    solve(template, rules, 40)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
    );
    assert_eq!(part1(input.clone()), 1588);
    assert_eq!(part2(input), 2188189693529);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2937);
    assert_eq!(part2(input), 3390034818249);
}
