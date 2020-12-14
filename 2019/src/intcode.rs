use std::convert::TryFrom;

pub fn cast(num: i64) -> usize {
    usize::try_from(num).unwrap()
}

pub fn parse_program(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug)]
pub enum State {
    Waiting,
    Yielded(i64),
    Complete,
}
