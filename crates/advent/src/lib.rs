mod human;
mod stats;

use std::env;
use std::fmt::Display;
use std::time::{Duration, Instant};

use peter::Stylize;

use crate::stats::Stats;

type FnBox<'a> = Box<dyn Fn() -> Box<dyn Display + 'a> + 'a>;

#[derive(Default)]
pub struct Advent<'a> {
    parts: Vec<(Option<String>, FnBox<'a>)>,
}

fn print_bench_summary(i: usize, name: Option<String>, times: &[f64]) {
    if i != 0 {
        println!();
    }
    let name = name.unwrap_or_else(|| format!("Part {}", i + 1));
    println!(
        "{}{:>width$}",
        name.bold(),
        human::fmt_samples(times.len()).fixed(245),
        width = 46 - name.chars().count(),
    );
    let (mean, std_dev, min, max) =
        human::fmt_time_four(times.mean(), times.std_dev(), times.min(), times.max());
    println!(
        "  Time ({} ± {}):     {:>10} ± {:>10}",
        "mean".green().bold(),
        "σ".green(),
        mean.green().bold(),
        std_dev.green(),
    );
    println!(
        "  Range ({} … {}):   {:>10} … {:>10}",
        "min".cyan(),
        "max".magenta(),
        min.cyan(),
        max.magenta(),
    );
}

fn print_run_summary(i: usize, name: Option<String>, result: String, elapsed: String) {
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
            print_run_summary(i, name, result.to_string(), human::fmt_time(elapsed));
        }
    }

    pub fn bench(self) {
        for (i, (name, f)) in self.parts.into_iter().enumerate() {
            let five_s = Duration::from_secs(3);
            let three_s = Duration::from_secs(5);

            // warm up for 3 secs
            let start = Instant::now();
            while Instant::now() - start < five_s {
                drop(f());
            }

            // now time for 5 secs, but with at least 25 samples
            let mut times = Vec::new();
            let start = Instant::now();
            while times.len() < 25 || (Instant::now() - start < three_s && times.len() < 123_456) {
                let t0 = Instant::now();
                drop(f());
                let t1 = Instant::now();
                times.push((t1 - t0).as_secs_f64());
            }

            // remove extreme outliers 🤷‍♂️
            times.sort_by(stats::cmp);
            if times.len() > 1_000 {
                let min = times.percentile(1.0);
                let max = times.percentile(99.0);
                times.retain(|&t| t >= min && t <= max);
            }

            print_bench_summary(i, name, &times);
        }
    }

    pub fn finish(self) {
        match env::args().any(|arg| arg == "--bench") {
            true => {
                if cfg!(not(profile = "release")) {
                    panic!("--bench requires release mode");
                }
                self.bench()
            }
            false => {
                println!("{}", ascii_art::fun());
                self.once()
            }
        }
    }
}

pub fn start<'a>() -> Advent<'a> {
    Advent::default()
}
