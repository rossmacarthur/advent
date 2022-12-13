//! This crate contains a runner and benchmarker for Advent of Code solutions.
//!
//! # Getting started
//!
//! Add this crate to your Cargo manifest for your solution.
//!
//! ```toml
//! advent = { git = "https://github.com/rossmacarthur/advent" }
//! ```
//!
//! Then use the following as your main function.
//!
//! ```ignore
//! fn main() {
//!     let mut run = advent::with(parse_input);
//!     run.part(part1);
//!     run.part(part2);
//!     run.finish();
//! }
//! ```
//!
//! They can be enabled in your Cargo manifest like this:
//!
//! ```toml
//! [dependencies]
//! advent = { git = "https://github.com/rossmacarthur/advent", features = ["festive", "json"] }
//! ```
//!
//! **Where**
//!
//! - `parse_input` is a function that returns any type `I` implementing
//!   `Clone`.
//! - Each part function takes `I` as an argument and returns something
//!   implementing `Display`.
//!
//! Finally, `finish()` will instantiate a command line interface and run the
//! program. Ordinary runs will run each part once and output the answers.
//! Passing `--bench` to the program will perform a benchmark. That's all!
//! You're free to structure your program however else you want.
//!
//! # Features
//!
//! There are also some optional features which pull in some other crates.
//!
//! - **`festive`** enables some festive ascii art and changes the default
//!   output to `--output festive`
//! - **`json`** supports JSON output using `--output json`, useful for
//!   collecting benchmark information
//! - **`prelude`** re-exports my prelude crate that can be imported using
//!   ```rust
//!   use advent::prelude::*;
//!   ```
//!
//! They can be enabled in your Cargo manifest like this:
//!
//! ```toml
//! [dependencies]
//! advent = { git = "https://github.com/rossmacarthur/advent", features = ["festive", "json"] }
//! ```
//!
//! # CLI
//!
//! The command line interface looks like this.
//!
//! ```text
//! Usage: example [--bench] [--output <output>]
//!
//! Run the program.
//!
//! Options:
//!   --bench           whether to benchmark
//!   --output          the output style
//!   --help            display usage information
//! ```
//!
//! The output style can be `boring`, `festive`, or `json`. To use json this
//! crate requires the `json` feature to be set.
//!

mod human;
mod stats;
pub mod summary;

use std::fmt::Display;
use std::mem;
use std::panic::UnwindSafe;
use std::ptr;
use std::time::{Duration, Instant};

use argh::FromArgs;
#[cfg(feature = "prelude")]
pub use prelude;
use yansi::Paint;

use crate::summary::{Bench, Run, Summary};

type FnParse<'a, I> = Box<dyn Fn() -> I + 'a>;
type FnPart<'a, I> = Box<dyn Fn(I) -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

/// A runner and benchmarker for an Advent of Code solution.
pub struct Advent<'a, I> {
    parse: FnParse<'a, I>,
    parse_ok: bool,
    parts: Vec<(Option<String>, FnPart<'a, I>)>,
}

#[doc(hidden)]
pub fn new<'a>() -> Advent<'a, ()> {
    Advent {
        parse: Box::new(|| ()),
        parse_ok: false,
        parts: Vec::new(),
    }
}

/// Starts a new Advent of Code run or benchmark with the given input function.
///
/// # Examples
///
/// ```ignore
/// let run = advent::with(parse_input);
/// ```
pub fn with<'a, F, I>(parse: F) -> Advent<'a, I>
where
    F: Fn() -> I + UnwindSafe + 'a,
{
    Advent {
        parse: Box::new(parse),
        parse_ok: true,
        parts: Vec::new(),
    }
}

impl<'a, I> Advent<'a, I>
where
    I: Clone + UnwindSafe,
{
    /// Adds a part to run or benchmark.
    ///
    /// The closure must take the parsed input as a parameter and return a
    /// result that implements [`Display`]. The given function is simply stored.
    /// Nothing will happen until [`finish()`][Advent::finish] is called.
    pub fn part<F, R>(&mut self, f: F)
    where
        R: Display + 'a,
        F: Fn(I) -> R + UnwindSafe + 'a,
    {
        self.parts.push((None, Box::new(move |i| Box::new(f(i)))))
    }

    #[doc(hidden)]
    pub fn named<F, R>(&mut self, name: &str, f: F)
    where
        R: Display + 'a,
        F: Fn() -> R + UnwindSafe + 'a,
    {
        let name = Some(String::from(name));
        self.parts.push((name, Box::new(move |_| Box::new(f()))))
    }

    /// Consumes this struct and runs the parts.
    #[must_use]
    pub fn run(self) -> Summary {
        let mut runs = Vec::new();

        let input = (self.parse)();

        // Time each part
        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
            let input = input.clone();

            let (result, elapsed) = {
                let t0 = Instant::now();
                let result = std::panic::catch_unwind(move || f(input));
                let t1 = Instant::now();
                let elapsed = (t1 - t0).as_secs_f64();
                let result = match result {
                    Ok(result) => result.to_string(),
                    Err(_) => "🚨👻🚨".to_owned(),
                };
                (result, elapsed)
            };

            runs.push(Run {
                name,
                result,
                elapsed,
            })
        }

        Summary::Run(runs)
    }

    /// Consumes this struct and benchmarks the parts.
    #[must_use]
    pub fn bench(self) -> Summary {
        let mut benches = Vec::new();

        // Benchmark the parsing
        if self.parse_ok {
            let stats = bench(&self.parse);
            benches.push(Bench {
                name: "Parse".to_owned(),
                stats,
            });
        }

        // Benchmark each part
        let input = (self.parse)();
        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
            let stats = bench_with_input(input.clone(), &f);
            benches.push(Bench { name, stats });
        }

        Summary::Bench(benches)
    }

    /// Parses the command line arguments and executes the run or benchmark.
    pub fn finish(self) {
        let Opt { bench, output } = argh::from_env();

        #[cfg(feature = "festive")]
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

fn bench<F, O>(f: F) -> summary::Stats
where
    F: Fn() -> O,
{
    bench_with_input((), move |()| f())
}

fn bench_with_input<F, I, O>(input: I, f: F) -> summary::Stats
where
    I: Clone,
    F: Fn(I) -> O,
{
    const FIVE_SECS: Duration = Duration::from_secs(5);
    const THREE_SECS: Duration = Duration::from_secs(3);

    // warm up for 3 secs
    let start = Instant::now();
    while Instant::now() - start < THREE_SECS {
        black_box(f(input.clone()));
    }

    // now time for 5 secs, but with at least 25 samples
    let mut times = Vec::new();
    let start = Instant::now();
    while times.len() < 25 || (Instant::now() - start < FIVE_SECS && times.len() < 123_456) {
        let input = input.clone();
        let t0 = Instant::now();
        black_box(f(input));
        let t1 = Instant::now();
        times.push((t1 - t0).as_secs_f64());
    }

    stats::basics(times)
}

/// A function that is opaque to the optimizer, used to prevent the compiler
/// from optimizing away computations in a benchmark.
fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = ptr::read_volatile(&dummy);
        mem::forget(dummy);
        ret
    }
}

/// Run the program.
#[derive(Debug, FromArgs)]
struct Opt {
    /// whether to benchmark
    #[argh(switch)]
    bench: bool,
    /// the output style
    #[argh(option, default = "default_output()")]
    output: Output,
}

#[cfg(feature = "festive")]
fn default_output() -> Output {
    Output::Festive
}

#[cfg(not(feature = "festive"))]
fn default_output() -> Output {
    Output::Boring
}

#[derive(Debug)]
enum Output {
    Boring,
    #[cfg(feature = "festive")]
    Festive,
    #[cfg(feature = "json")]
    Json,
}

impl argh::FromArgValue for Output {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "boring" => Ok(Self::Boring),
            "festive" => {
                #[cfg(feature = "festive")]
                {
                    Ok(Self::Festive)
                }
                #[cfg(not(feature = "festive"))]
                {
                    Err("`festive` requires crate feature".into())
                }
            }
            "json" => {
                #[cfg(feature = "json")]
                {
                    Ok(Self::Json)
                }
                #[cfg(not(feature = "json"))]
                {
                    Err("`json` requires crate feature".into())
                }
            }
            _ => Err("expected `boring`, `festive` or `json`".into()),
        }
    }
}
