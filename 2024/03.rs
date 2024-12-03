use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 03)
}

fn part1(input: &str) -> i64 {
    regex!(r"mul\((\d+),(\d+)\)")
        .captures_iter(input)
        .map(|m| {
            let a = m[1].parse::<i64>().unwrap();
            let b = m[2].parse::<i64>().unwrap();
            a * b
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    enum State {
        Do,
        Dont,
    }
    regex!(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)")
        .captures_iter(input)
        .scan(State::Do, |state, m| {
            match &m[0] {
                "do()" => *state = State::Do,
                "don't()" => *state = State::Dont,
                _ => {
                    if let State::Do = state {
                        let a = m[1].parse::<i64>().unwrap();
                        let b = m[2].parse::<i64>().unwrap();
                        return Some(a * b);
                    }
                }
            }
            Some(0)
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
}

#[test]
fn example2() {
    let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 167650499);
    assert_eq!(part2(input), 95846796);
}
