use regex_macro::regex;

fn parse_input(s: &str) -> Vec<Element> {
    regex!(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>.):\s(?P<password>.*)")
        .captures_iter(s)
        .map(|caps| {
            let lower = caps["lower"].parse().unwrap();
            let upper = caps["upper"].parse().unwrap();
            let letter = caps["letter"].parse().unwrap();
            let password = caps["password"].to_owned();
            Element {
                lower,
                upper,
                letter,
                password,
            }
        })
        .collect()
}

fn default_input() -> Vec<Element> {
    parse_input(include_str!("input/02.txt"))
}

struct Element {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

fn part1(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            let freq = e.password.chars().filter(|&c| c == e.letter).count();
            (e.lower..=e.upper).contains(&freq)
        })
        .count()
}

fn part2(elements: &[Element]) -> usize {
    elements
        .iter()
        .filter(|e| {
            (e.password.chars().nth(e.lower - 1).unwrap() == e.letter)
                ^ (e.password.chars().nth(e.upper - 1).unwrap() == e.letter)
        })
        .count()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc",
    );
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 1);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 383);
    assert_eq!(part2(&input), 272);
}
