//! The latest and greatest intcode computer.

#![allow(dead_code)]

use std::cmp::max;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;
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

/// Something that can provide input to the computer.
pub trait Input {
    fn input(self, input: &mut VecDeque<i64>);
}

/// The state of the computer.
#[derive(Debug)]
pub enum State<T> {
    /// An output.
    Yielded(T),
    /// Waiting for input.
    Waiting,
    /// Program execution has finished.
    Complete,
}

#[derive(Debug)]
pub struct Computer {
    mem: Vec<i64>,
    ptr: usize,
    relative_base: i64,
    input: VecDeque<i64>,
}

impl Input for i64 {
    fn input(self, v: &mut VecDeque<i64>) {
        v.push_back(self);
    }
}

impl Input for (i64, i64) {
    fn input(self, v: &mut VecDeque<i64>) {
        v.push_back(self.0);
        v.push_back(self.1);
    }
}

impl Input for Vec<i64> {
    fn input(self, v: &mut VecDeque<i64>) {
        v.extend(self);
    }
}

impl<T> State<T> {
    pub fn map<U, F>(self, op: F) -> State<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Yielded(t) => State::Yielded(op(t)),
            Self::Waiting => State::Waiting,
            Self::Complete => State::Complete,
        }
    }

    pub const fn as_ref(&self) -> State<&T> {
        match *self {
            Self::Yielded(ref o) => State::Yielded(o),
            Self::Waiting => State::Waiting,
            Self::Complete => State::Complete,
        }
    }

    pub fn as_deref(&self) -> State<&T::Target>
    where
        T: Deref,
    {
        self.as_ref().map(T::deref)
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Yielded(output) => output,
            Self::Waiting => panic!("called `State::unwrap()` on a `Waiting` state"),
            Self::Complete => panic!("called `State::unwrap()` on a `Complete` state"),
        }
    }
}

impl Computer {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            mem: program,
            ptr: 0,
            relative_base: 0,
            input: VecDeque::new(),
        }
    }

    fn mem_get(&self, addr: usize) -> i64 {
        self.mem.get(addr).copied().unwrap_or(0)
    }

    pub fn mem_get_mut(&mut self, addr: usize) -> &mut i64 {
        self.mem.resize(max(self.mem.len(), addr + 1), 0);
        &mut self.mem[addr]
    }

    fn param_ptr(&self, i: usize) -> usize {
        let opcode = self.mem_get(self.ptr);
        let ptr = self.ptr + i;
        match opcode / (10i64.pow((1 + i) as u32)) % 10 {
            0 => cast(self.mem_get(ptr)),
            1 => ptr,
            2 => cast(self.relative_base + self.mem_get(ptr)),
            mode => panic!("unknown mode `{}`", mode),
        }
    }

    fn param(&self, i: usize) -> i64 {
        self.mem_get(self.param_ptr(i))
    }

    fn param_mut(&mut self, i: usize) -> &mut i64 {
        self.mem_get_mut(self.param_ptr(i))
    }

    pub fn input<I: Input>(&mut self, i: I) -> &mut Self {
        i.input(&mut self.input);
        self
    }

    pub fn next(&mut self) -> State<i64> {
        loop {
            match self.mem_get(self.ptr) % 100 {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = self.input.pop_front() {
                        *self.param_mut(1) = input;
                        self.ptr += 2;
                    } else {
                        break State::Waiting;
                    }
                }
                4 => {
                    let output = self.param(1);
                    self.ptr += 2;
                    break State::Yielded(output);
                }
                5 => {
                    if self.param(1) != 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if self.param(1) == 0 {
                        self.ptr = cast(self.param(2));
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    *self.param_mut(3) = (self.param(1) < self.param(2)) as i64;
                    self.ptr += 4;
                }
                8 => {
                    *self.param_mut(3) = (self.param(1) == self.param(2)) as i64;
                    self.ptr += 4;
                }
                9 => {
                    self.relative_base += self.param(1);
                    self.ptr += 2;
                }
                99 => break State::Complete,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }

    pub fn next_value(&mut self) -> Option<i64> {
        match self.next() {
            State::Yielded(v) => Some(v),
            State::Complete => None,
            state => panic!("unexpected state `{:?}`", state),
        }
    }
}
