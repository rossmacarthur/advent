pub mod prelude;
mod stats;
mod summary;

use std::fmt::Display;
use std::panic::UnwindSafe;
use std::time::{Duration, Instant};

use argh::FromArgs;
use yansi::Paint;

pub use crate::summary::Summary;
use crate::summary::{Bench, Run};

type FnBox<'a> = Box<dyn Fn() -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

#[derive(Default)]
pub struct Advent<'a> {
    parts: Vec<(Option<String>, FnBox<'a>)>,
}

pub fn start<'a>() -> Advent<'a> {
    Advent::default()
}

impl<'a> Advent<'a> {
    pub fn part<F, R>(&mut self, f: F)
    where
        R: Display + 'a,
        F: Fn() -> R + UnwindSafe + 'a,
    {
        self.parts.push((None, Box::new(move || Box::new(f()))))
    }

    pub fn named<F, R>(&mut self, name: &str, f: F)
    where
        R: Display + 'a,
        F: Fn() -> R + UnwindSafe + 'a,
    {
        let name = Some(String::from(name));
        self.parts.push((name, Box::new(move || Box::new(f()))))
    }

    pub fn run(self) -> Summary {
        let mut runs = Vec::new();

        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let start = Instant::now();
            let result = std::panic::catch_unwind(f);
            let elapsed = (Instant::now() - start).as_secs_f64();
            let result = match result {
                Ok(result) => result.to_string(),
                Err(_) => "🚨👻🚨".to_owned(),
            };
            runs.push(Run {
                name: name.unwrap_or_else(|| format!("Part {}", i + 1)),
                result,
                elapsed,
            })
        }

        Summary::Run(runs)
    }

    pub fn bench(self) -> Summary {
        let mut benches = Vec::new();

        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            const FIVE_SECS: Duration = Duration::from_secs(5);
            const THREE_SECS: Duration = Duration::from_secs(3);

            // warm up for 3 secs
            let start = Instant::now();
            while Instant::now() - start < FIVE_SECS {
                drop(f());
            }

            // now time for 5 secs, but with at least 25 samples
            let mut times = Vec::new();
            let start = Instant::now();
            while times.len() < 25 || (Instant::now() - start < THREE_SECS && times.len() < 123_456)
            {
                let t0 = Instant::now();
                drop(f());
                let t1 = Instant::now();
                times.push((t1 - t0).as_secs_f64());
            }

            benches.push(Bench {
                name: name.unwrap_or_else(|| format!("Part {}", i + 1)),
                stats: stats::basics(times),
            });
        }

        Summary::Bench(benches)
    }

    pub fn finish(self) {
        let Opt { bench, output } = argh::from_env();

        if let Output::Festive = output {
            println!("{}", ascii_art::fun());
        }
        let summary = if bench {
            if cfg!(not(profile = "release")) {
                eprintln!(
                    "{}\n",
                    Paint::yellow("Note: using --bench without --release").bold()
                );
            }
            self.bench()
        } else {
            self.run()
        };

        match output {
            #[cfg(feature = "json")]
            Output::Json => summary.print_json(),
            _ => summary.print(),
        }
    }
}

/// Run the program.
#[derive(Debug, FromArgs)]
struct Opt {
    /// whether to benchmark
    #[argh(switch)]
    bench: bool,
    /// the output style
    #[argh(option, default = "Output::Festive")]
    output: Output,
}

#[derive(Debug)]
enum Output {
    Boring,
    Festive,
    #[cfg(feature = "json")]
    Json,
}

impl argh::FromArgValue for Output {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "boring" => Ok(Self::Boring),
            "festive" => Ok(Self::Festive),
            #[cfg(feature = "json")]
            "json" => Ok(Self::Json),
            #[cfg(not(feature = "json"))]
            "json" => Err("`json` requires crate feature".into()),
            _ => Err("expected `boring`, `festive` or `json`".into()),
        }
    }
}
