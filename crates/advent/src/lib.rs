mod human;
mod output;
pub mod prelude;
mod stats;
mod types;

use std::fmt::Display;
use std::io;
use std::time::{Duration, Instant};

use argh::FromArgs;
use peter::Stylize;

use crate::types::{Bench, Run, Summary};

type FnBox<'a> = Box<dyn Fn() -> Box<dyn Display + 'a> + 'a>;

#[derive(Default)]
pub struct Advent<'a> {
    parts: Vec<(Option<String>, FnBox<'a>)>,
}

impl<'a> Advent<'a> {
    pub fn part<F, R>(&mut self, f: F)
    where
        R: Display + 'a,
        F: Fn() -> R + 'a,
    {
        self.parts.push((None, Box::new(move || Box::new(f()))))
    }

    pub fn named<F, R>(&mut self, name: &str, f: F)
    where
        R: Display + 'a,
        F: Fn() -> R + 'a,
    {
        let name = Some(String::from(name));
        self.parts.push((name, Box::new(move || Box::new(f()))))
    }

    fn run(self) -> Summary {
        let mut runs = Vec::new();

        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let start = Instant::now();
            let result = f();
            let elapsed = (Instant::now() - start).as_secs_f64();
            runs.push(Run {
                name: name.unwrap_or_else(|| format!("Part {}", i + 1)),
                result: result.to_string(),
                elapsed,
            })
        }

        Summary::Run(runs)
    }

    fn bench(self) -> Summary {
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
        /// Run the program.
        #[derive(Debug, FromArgs)]
        struct Opt {
            /// whether to benchmark
            #[argh(switch)]
            bench: bool,
            /// whether to not print trees
            #[argh(switch)]
            no_trees: bool,
        }

        let Opt { bench, no_trees } = argh::from_env();
        if !no_trees {
            println!("{}", ascii_art::fun());
        }
        let summary = if bench {
            if cfg!(not(profile = "release")) {
                eprintln!(
                    "{}\n",
                    "Note: using --bench without --release".yellow().bold()
                );
            }
            self.bench()
        } else {
            self.run()
        };

        output::print_summary(io::stdout(), &summary).unwrap();
    }
}

pub fn start<'a>() -> Advent<'a> {
    Advent::default()
}
