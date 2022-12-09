pub mod prelude;
mod stats;
mod summary;

use std::fmt::Display;
use std::mem;
use std::panic::UnwindSafe;
use std::ptr;
use std::time::{Duration, Instant};

use argh::FromArgs;
use yansi::Paint;

pub use crate::summary::Summary;
use crate::summary::{Bench, Run};

////////////////////////////////////////////////////////////////////////////////
/// New runner
////////////////////////////////////////////////////////////////////////////////

type FnParse<'a, I> = Box<dyn Fn() -> I + 'a>;
type FnPart<'a, I> = Box<dyn Fn(I) -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

pub struct Advent<'a, I> {
    parse: Option<FnParse<'a, I>>,
    parts: Vec<(Option<String>, FnPart<'a, I>)>,
}

pub fn new<'a, I>() -> Advent<'a, I> {
    Advent {
        parse: None,
        parts: Vec::new(),
    }
}

impl<'a, I> Advent<'a, I>
where
    I: Clone + UnwindSafe,
{
    pub fn part<F, R>(&mut self, f: F)
    where
        R: Display + 'a,
        F: Fn(I) -> R + UnwindSafe + 'a,
    {
        self.parts.push((None, Box::new(move |i| Box::new(f(i)))))
    }

    pub fn named<F, R>(&mut self, name: &str, f: F)
    where
        R: Display + 'a,
        F: Fn(I) -> R + UnwindSafe + 'a,
    {
        let name = Some(String::from(name));
        self.parts.push((name, Box::new(move |i| Box::new(f(i)))))
    }

    pub fn input<F>(&mut self, f: F)
    where
        F: Fn() -> I + UnwindSafe + 'a,
    {
        self.parse = Some(Box::new(f))
    }

    pub fn run(self) -> Summary {
        let mut runs = Vec::new();

        let input = self.parse.expect("no input provided")();

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

    pub fn bench(self) -> Summary {
        let mut benches = Vec::new();

        let parse_fn = self.parse.expect("no input provided");

        // Benchmark the parsing
        let stats = bench(&parse_fn);
        benches.push(Bench {
            name: "Parse".to_owned(),
            stats,
        });

        // Benchmark each part
        let input = parse_fn();
        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
            let stats = bench_with_input(input.clone(), &f);
            benches.push(Bench { name, stats });
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

////////////////////////////////////////////////////////////////////////////////
/// Old runner
////////////////////////////////////////////////////////////////////////////////

type FnBox<'a> = Box<dyn Fn() -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

#[derive(Default)]
pub struct AdventOld<'a> {
    parts: Vec<(Option<String>, FnBox<'a>)>,
}

pub fn start<'a>() -> AdventOld<'a> {
    AdventOld::default()
}

impl<'a> AdventOld<'a> {
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
