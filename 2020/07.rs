use advent::prelude::*;

type Bags<'a> = HashMap<&'a str, Vec<(&'a str, i64)>>;

fn parse_input(input: &str) -> Bags<'_> {
    regex!(r"(?P<color>\w+ \w+) bags contain (?P<contents>.*)\.")
        .captures_iter(input)
        .map(|caps| {
            let bag = caps.name("color").unwrap().as_str();
            let contents = caps.name("contents").unwrap().as_str();
            let contents = regex!(r"(?P<count>\d+) (?P<color>\w+ \w+) bags?")
                .captures_iter(contents)
                .map(|captures| {
                    let count = captures.name("count").unwrap().as_str().parse().unwrap();
                    let bag = captures.name("color").unwrap().as_str();
                    (bag, count)
                })
                .collect();
            (bag, contents)
        })
        .collect()
}

fn default_input() -> Bags<'static> {
    parse_input(include_str!("input/07.txt"))
}

fn contains(bags: &Bags<'_>, bag: &str) -> bool {
    bags[bag]
        .iter()
        .any(|&(b, _)| b == "shiny gold" || contains(bags, b))
}

fn count(bags: &Bags<'_>, bag: &str) -> i64 {
    bags[bag]
        .iter()
        .map(|(b, i)| i * (1 + count(bags, b)))
        .sum()
}

fn part1(bags: Bags<'_>) -> usize {
    bags.keys().filter(|b| contains(&bags, b)).count()
}

fn part2(bags: Bags<'_>) -> i64 {
    count(&bags, "shiny gold")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#,
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 32);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#,
    );
    assert_eq!(part1(input.clone()), 0);
    assert_eq!(part2(input), 126);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 101);
    assert_eq!(part2(input), 108636);
}
