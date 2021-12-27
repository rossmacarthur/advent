mod human;
pub mod prelude;
mod stats;

use std::fmt::Display;
use std::time::{Duration, Instant};

use argh::FromArgs;
use peter::Stylize;

type FnBox<'a> = Box<dyn Fn() -> Box<dyn Display + 'a> + 'a>;

#[derive(Default)]
pub struct Advent<'a> {
    parts: Vec<(Option<String>, FnBox<'a>)>,
}

fn print_bench_summary(i: usize, name: Option<String>, stats: stats::Stats) {
    if i != 0 {
        println!();
    }
    let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
    println!(
        "{}{:>width$}",
        name.bold(),
        human::Samples::new(stats.len).fixed(245),
        width = 46 - name.chars().count(),
    );
    let mean = human::Time::new(stats.mean);
    let std_dev = human::Time::with_scale(stats.std_dev, mean.scale());
    let min = human::Time::with_scale(stats.min, mean.scale());
    let max = human::Time::with_scale(stats.max, mean.scale());
    println!(
        "  Time ({} ± {}):       {:>9} ± {:>9}",
        "mean".green().bold(),
        "σ".green(),
        mean.green().bold(),
        std_dev.green(),
    );
    println!(
        "  Range ({} … {}):     {:>9} … {:>9}",
        "min".cyan(),
        "max".magenta(),
        min.cyan(),
        max.magenta(),
    );
}

fn print_run_summary(i: usize, name: Option<String>, result: String, elapsed: human::Time) {
    if i != 0 {
        println!();
    }
    let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
    let width = 46_usize.saturating_sub(name.chars().count() + 2);
    println!(
        "{}: {:>width$}\n{}",
        name.bold().cyan(),
        format!("({})", elapsed).fixed(245),
        result.bold(),
        width = width,
    )
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

    fn once(self) {
        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let start = Instant::now();
            let result = f();
            let elapsed = (Instant::now() - start).as_secs_f64();
            print_run_summary(i, name, result.to_string(), human::Time::new(elapsed));
        }
    }

    pub fn bench(self) {
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

            print_bench_summary(i, name, stats::basics(times));
        }
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
        if bench {
            if cfg!(not(profile = "release")) {
                eprintln!(
                    "{}\n",
                    "Note: using --bench without --release".yellow().bold()
                );
            }
            self.bench()
        } else {
            self.once()
        }
    }
}

pub fn start<'a>() -> Advent<'a> {
    Advent::default()
}
