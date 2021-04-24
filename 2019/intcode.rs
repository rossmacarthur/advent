#![allow(dead_code)]

use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

pub fn cast(num: i64) -> usize {
    usize::try_from(num).unwrap()
}

pub fn parse_program<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
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

impl State {
    #[track_caller]
    pub fn unwrap(self) -> i64 {
        match self {
            Self::Yielded(output) => output,
            state => panic!("called `State::unwrap()` on a `{:?}` state", state),
        }
    }
}
