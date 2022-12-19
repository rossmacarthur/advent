//! Format numbers for humans.

use std::cmp::max;
use std::fmt;

/// A number that is scaled in an easy to read way.
#[derive(Debug, Clone, Copy)]
pub struct Number(f64, Scale);

/// The scale to represent the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Nano,
    Micro,
    Milli,
    Unit,
    Kilo,
    Mega,
    Giga,
}

/// Represents the time taken.
#[derive(Debug, Clone, Copy)]
pub struct Time(Number);

/// Represents the number of samples.
#[derive(Debug, Clone, Copy)]
pub struct Samples(Number);

impl Number {
    pub fn new(v: f64) -> Self {
        let scales = [
            (Scale::Giga, 1e-9),
            (Scale::Mega, 1e-6),
            (Scale::Kilo, 1e-3),
            (Scale::Unit, 1e0),
            (Scale::Milli, 1e3),
            (Scale::Micro, 1e6),
            (Scale::Nano, 1e9),
        ];
        let scale = scales
            .into_iter()
            .find_map(|(s, f)| if v * f >= 1.0 { Some(s) } else { None })
            .unwrap_or(Scale::Nano);
        Self::with_scale(v, scale)
    }

    pub fn with_scale(v: f64, scale: Scale) -> Self {
        let v = match scale {
            Scale::Nano => v * 1e9,
            Scale::Micro => v * 1e6,
            Scale::Milli => v * 1e3,
            Scale::Unit => v * 1e0,
            Scale::Kilo => v * 1e-3,
            Scale::Mega => v * 1e-6,
            Scale::Giga => v * 1e-9,
        };
        Self(v, scale)
    }
}

impl Time {
    pub fn new(secs: f64) -> Self {
        Self(Number::new(secs))
    }

    pub fn with_scale(secs: f64, scale: Scale) -> Self {
        Self(Number::with_scale(secs, scale))
    }

    pub fn scale(&self) -> Scale {
        let Self(Number(_, scale)) = self;
        *scale
    }
}

impl Samples {
    pub fn new(n: usize) -> Self {
        Self(Number::new(n as f64))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Some examples of what we need to handle nicely
        //
        // - 500 s
        // - 180.4 s
        // - 1.234 s
        // - 42.53 ms
        // - 978.1 µs
        //
        // Rules:
        // - If very large >= 300 then use no precision
        // - Right align number in a width of 5 characters and fill the space
        //   if possible

        let &Self(Number(v, s)) = self;
        let p = 4 - digits(v);
        let (precision, v, suffix) = match s {
            Scale::Nano => (p, v, " ns"),
            Scale::Micro => (p, v, " µs"),
            Scale::Milli => (p, v, " ms"),
            Scale::Unit => (if v >= 300.0 { 0 } else { p }, v, " s"),
            Scale::Kilo => (0, v * 1e3, " s"),
            Scale::Mega => (0, v * 1e6, " s"),
            Scale::Giga => (0, v * 1e9, " s"),
        };
        fmt::Display::fmt(&format!("{v:.precision$}{suffix}"), f)
    }
}

impl fmt::Display for Samples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Some examples of what we need to handle nicely
        //
        // - 11 samples
        // - 1.234k samples
        // - 10.48M samples
        // - 123.1G samples
        //
        // Rules:
        // - If very small < 1000 then use no precision
        // - Right align number in a width of 5 characters and fill the space.

        let &Self(Number(v, s)) = self;
        let p = 4 - digits(v);
        let (precision, v, suffix) = match s {
            Scale::Unit => (if v < 1000.0 { 0 } else { p }, v, " samples"),
            Scale::Kilo => (p, v, "k samples"),
            Scale::Mega => (p, v, "M samples"),
            Scale::Giga => (p, v, "G samples"),
            _ => unreachable!(),
        };
        fmt::Display::fmt(&format!("{v:.precision$}{suffix}"), f)
    }
}

fn digits(mut v: f64) -> usize {
    let mut n = 0;
    while v >= 1. {
        v *= 0.1;
        n += 1;
    }
    max(n, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_digits() {
        assert_eq!(digits(1.0), 1);
        assert_eq!(digits(0.123), 1);
        assert_eq!(digits(10.0), 2);
        assert_eq!(digits(15.0), 2);
        assert_eq!(digits(199.99), 3);
        assert_eq!(digits(123.123), 3);
    }

    #[test]
    fn time_display() {
        let test_cases = [
            (500.0, "500 s"),
            (180.4, "180.4 s"),
            (1.234, "1.234 s"),
            (0.04253, "42.53 ms"),
            (0.0009781, "978.1 µs"),
        ];
        for (t, expected) in test_cases {
            assert_eq!(Time::new(t).to_string(), expected);
        }
    }

    #[test]
    fn samples_display() {
        let test_cases = [
            (3, "3 samples"),
            (11, "11 samples"),
            (123, "123 samples"),
            (1337, "1.337k samples"),
            (23_123, "23.12k samples"),
        ];
        for (t, expected) in test_cases {
            assert_eq!(Samples::new(t).to_string(), expected);
        }
    }
}
