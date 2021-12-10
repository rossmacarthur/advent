use itertools::Itertools;

fn default_input() -> &'static str {
    include_str!("input/10.txt")
}

#[derive(Debug)]
enum State {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn validate_line(line: &str) -> State {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            open @ ('(' | '[' | '{' | '<') => stack.push(open),
            close @ (')' | ']' | '}' | '>') => {
                let open = stack.pop().unwrap();
                match (open, close) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                    _ => return State::Corrupted(close),
                }
            }
            c => panic!("unexpected character `{}`", c),
        }
    }
    match stack.is_empty() {
        true => State::Valid,
        false => State::Incomplete(stack),
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(validate_line)
        .filter_map(|state| match state {
            State::Corrupted(bad) => Some(bad),
            _ => None,
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let scores: Vec<_> = input
        .lines()
        .map(validate_line)
        .filter_map(|state| match state {
            State::Incomplete(unmatched) => Some(unmatched),
            _ => None,
        })
        .map(|unmatched| {
            unmatched.into_iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .sorted_unstable()
        .collect();
    scores[scores.len() / 2]
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(part1(input), 26397);
    assert_eq!(part2(input), 288957);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 367059);
    assert_eq!(part2(input), 1952146692);
}
