use std::fmt;

const THOUSAND: f64 = 1_000.;
const MILLION: f64 = 1_000_000.;
const BILLION: f64 = 1_000_000_000.;

const MILLIS_PER_SEC: f64 = THOUSAND;
const MICROS_PER_SEC: f64 = MILLION;
const NANOS_PER_SEC: f64 = BILLION;

#[derive(Debug, Clone, Copy)]
pub enum SampleUnit {
    Single,
    Thousand,
    Million,
    Billion,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    Secs,
    Millis,
    Micros,
    Nanos,
}

impl SampleUnit {
    fn from_samples(f: f64) -> Self {
        if f >= BILLION {
            Self::Billion
        } else if f >= MILLION {
            Self::Million
        } else if f >= THOUSAND {
            Self::Thousand
        } else {
            Self::Single
        }
    }

    fn scale(&self, f: f64) -> f64 {
        match *self {
            Self::Single => f,
            Self::Thousand => f / THOUSAND,
            Self::Million => f / MILLION,
            Self::Billion => f / BILLION,
        }
    }
}

impl TimeUnit {
    fn from_secs(f: f64) -> Self {
        if f >= 1.0 {
            Self::Secs
        } else if f >= 1.0 / MILLIS_PER_SEC {
            Self::Millis
        } else if f >= 1.0 / MICROS_PER_SEC {
            Self::Micros
        } else {
            Self::Nanos
        }
    }

    fn scale(&self, f: f64) -> f64 {
        match *self {
            Self::Secs => f,
            Self::Millis => f * MILLIS_PER_SEC,
            Self::Micros => f * MICROS_PER_SEC,
            Self::Nanos => f * NANOS_PER_SEC,
        }
    }
}

impl fmt::Display for SampleUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match *self {
            Self::Single => " samples",
            Self::Thousand => "K samples",
            Self::Million => "M samples",
            Self::Billion => "B samples",
        };
        fmt::Display::fmt(unit, f)
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match *self {
            Self::Secs => "s",
            Self::Millis => "ms",
            Self::Micros => "µs",
            Self::Nanos => "ns",
        };
        fmt::Display::fmt(unit, f)
    }
}

pub fn fmt_samples(i: usize) -> String {
    let f = i as f64;
    let unit = SampleUnit::from_samples(f);
    if let SampleUnit::Single = unit {
        format!("{:.0}{}", unit.scale(f), unit)
    } else {
        format!("{:.2}{}", unit.scale(f), unit)
    }
}

pub fn fmt_time(f: f64) -> String {
    let unit = TimeUnit::from_secs(f);
    format!("{:.3} {}", unit.scale(f), unit)
}

pub fn fmt_time_four(a: f64, b: f64, c: f64, d: f64) -> (String, String, String, String) {
    let unit = TimeUnit::from_secs(a);
    macro_rules! fmt {
        ($i:ident) => {
            format!("{:.3} {}", unit.scale($i), unit)
        };
    }
    (fmt!(a), fmt!(b), fmt!(c), fmt!(d))
}
