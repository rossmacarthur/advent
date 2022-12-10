#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::human;

/// The summary of a set of runs or benchmarks.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Summary {
    #[cfg_attr(feature = "json", serde(rename = "benches"))]
    Bench(Vec<Bench>),
    #[cfg_attr(feature = "json", serde(rename = "runs"))]
    Run(Vec<Run>),
}

/// The result of a benchmark.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Bench {
    /// The name of the benchmark.
    pub name: String,
    /// The data for the benchmark.
    pub data: Data,
    /// The data for the benchmark, displayed nicely.
    pub human: Human,
}

/// The result of a run.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Run {
    /// The name of the run.
    pub name: String,
    /// The output of the run.
    pub result: String,
    /// How long this run took in seconds.
    pub elapsed: f64,
    /// How long this run took, displayed nicely.
    pub elapsed_human: String,
}

/// Data for a benchmark.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Data {
    /// The number of samples taken.
    pub samples: usize,
    /// The minimum time taken in seconds for a sample.
    pub min: f64,
    /// The maximum time taken in seconds for a sample.
    pub max: f64,
    /// The mean time taken in seconds for all samples.
    pub mean: f64,
    /// The standard deviation of time taken in seconds for all samples.
    pub std_dev: f64,
}

/// Data for the benchmark, displayed nicely.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Human {
    /// The number of samples taken.
    pub samples: String,
    /// The minimum time taken in seconds for a sample.
    pub min: String,
    /// The maximum time taken in seconds for a sample.
    pub max: String,
    /// The mean time taken in seconds for all samples.
    pub mean: String,
    /// The standard deviation of time taken in seconds for all samples.
    pub std_dev: String,
}

impl Bench {
    pub fn new(name: String, data: Data) -> Self {
        let human = {
            let samples = human::Samples::new(data.samples).to_string();
            let mean = human::Time::new(data.mean);
            let scale = mean.scale();
            let mean = mean.to_string();
            let std_dev = human::Time::with_scale(data.std_dev, scale).to_string();
            let min = human::Time::with_scale(data.min, scale).to_string();
            let max = human::Time::with_scale(data.max, scale).to_string();
            Human {
                samples,
                mean,
                min,
                max,
                std_dev,
            }
        };

        Self { name, data, human }
    }
}

impl Run {
    pub fn new(name: String, result: String, elapsed: f64) -> Self {
        let elapsed_human = human::Time::new(elapsed).to_string();
        Self {
            name,
            result,
            elapsed,
            elapsed_human,
        }
    }
}

impl Summary {
    pub fn print(&self) {
        match self {
            Self::Bench(parts) => print_bench_summary(parts),
            Self::Run(parts) => print_run_summary(parts),
        }
    }

    #[cfg(feature = "json")]
    pub fn print_json(&self) {
        let s = serde_json::to_string(self).unwrap();
        println!("{}", s);
    }
}

fn print_bench_summary(parts: &[Bench]) {
    for (i, part) in parts.iter().enumerate() {
        let Bench { name, human, .. } = part;
        if i != 0 {
            println!();
        }
        println!(
            "{}{:>width$}",
            Paint::new(name).bold(),
            Paint::fixed(245, &human.samples),
            width = 46 - name.chars().count(),
        );
        println!(
            "  Time ({} ± {}):       {:>9} ± {:>9}",
            Paint::green("mean").bold(),
            Paint::green("σ"),
            Paint::green(&human.mean).bold(),
            Paint::green(&human.std_dev),
        );
        println!(
            "  Range ({} … {}):     {:>9} … {:>9}",
            Paint::cyan("min"),
            Paint::magenta("max"),
            Paint::cyan(&human.min),
            Paint::magenta(&human.max),
        );
    }
}

fn print_run_summary(parts: &[Run]) {
    for (i, part) in parts.iter().enumerate() {
        let Run {
            name,
            result,
            elapsed_human,
            ..
        } = part;
        if i != 0 {
            println!();
        }
        let width = 46_usize.saturating_sub(name.chars().count() + 2);
        println!(
            "{}: {:>width$}\n{}",
            Paint::cyan(name).bold(),
            Paint::fixed(245, format!("({})", elapsed_human)),
            Paint::new(result).bold(),
            width = width,
        );
    }
}
