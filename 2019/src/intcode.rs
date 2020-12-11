use std::convert::TryFrom;

pub fn cast(num: i64) -> usize {
    usize::try_from(num).unwrap()
}

pub fn parse_program(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}
