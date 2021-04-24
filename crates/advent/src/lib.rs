use std::fmt::Display;
use std::time::Instant;

use peter::Stylize;

struct Bucket {
    title: &'static str,
    instant: Instant,
    result: Option<Box<dyn Display>>,
}

pub struct Advent {
    start: Instant,
    buckets: Vec<Bucket>,
}

impl Advent {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            buckets: Vec::new(),
        }
    }

    pub fn time<T>(&mut self, title: &'static str, t: T) -> T {
        self.buckets.push(Bucket {
            title,
            instant: Instant::now(),
            result: None,
        });
        t
    }

    pub fn result<R>(&mut self, title: &'static str, result: R)
    where
        R: Display + Sized + 'static,
    {
        let bucket = Bucket {
            title,
            instant: Instant::now(),
            result: Some(Box::new(result)),
        };
        self.buckets.push(bucket);
    }

    pub fn finish(self) {
        for (i, b) in self.buckets.iter().enumerate() {
            let prev = if i == 0 {
                self.start
            } else {
                self.buckets[i - 1].instant
            };
            let timing = format!(
                "({:.3}ms)",
                (b.instant - prev).as_nanos() as f64 / 1_000_000.0
            );
            match &b.result {
                Some(result) => {
                    println!(
                        "{}: {} {}",
                        b.title.bold().cyan(),
                        result.bold(),
                        timing.fixed(245),
                    );
                }
                None => {
                    println!("{} {}", b.title.bold().magenta(), timing.fixed(245));
                }
            }
        }
    }
}

pub fn start() -> Advent {
    Advent::new()
}
