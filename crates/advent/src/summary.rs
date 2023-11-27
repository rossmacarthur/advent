use yansi::Paint;

use crate::human;

/// The summary of a set of runs or benchmarks.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum Summary {
    #[cfg_attr(feature = "json", serde(rename = "benches"))]
    Bench(Vec<Bench>),
    #[cfg_attr(feature = "json", serde(rename = "runs"))]
    Run(Vec<Run>),
}

/// The result of a benchmark.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[non_exhaustive]
pub struct Bench {
    /// The name of the benchmark.
    pub name: String,
    /// The data for the benchmark.
    #[cfg_attr(feature = "json", serde(flatten))]
    pub stats: Stats,
}

/// The result of a run.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[non_exhaustive]
pub struct Run {
    /// The name of the run.
    pub name: String,
    /// The output of the run.
    pub result: String,
    /// How long this run took in seconds.
    pub elapsed: f64,
}

/// Data for a benchmark.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[non_exhaustive]
pub struct Stats {
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

impl Summary {
    /// Pretty prints the summary to stdout.
    pub fn print(&self) {
        match self {
            Self::Bench(parts) => print_bench_summary(parts),
            Self::Run(parts) => print_run_summary(parts),
        }
    }

    /// Prints the summary as JSON to stdout.
    #[cfg(feature = "json")]
    pub fn print_json(&self) -> serde_json::Result<()> {
        serde_json::to_writer(std::io::BufWriter::new(std::io::stdout()), self)
    }
}

fn print_bench_summary(parts: &[Bench]) {
    for (i, part) in parts.iter().enumerate() {
        let Bench { name, stats } = part;
        if i != 0 {
            println!();
        }
        println!(
            "{}{:>width$}",
            Paint::new(name).bold(),
            Paint::fixed(&human::Samples::new(stats.samples), 245),
            width = 46 - name.chars().count(),
        );
        let mean = human::Time::new(stats.mean);
        let std_dev = human::Time::with_scale(stats.std_dev, mean.scale());
        let min = human::Time::with_scale(stats.min, mean.scale());
        let max = human::Time::with_scale(stats.max, mean.scale());
        println!(
            "  Time ({} ± {}):       {:>9} ± {:>9}",
            Paint::green("mean").bold(),
            Paint::green("σ"),
            Paint::green(&mean).bold(),
            Paint::green(&std_dev),
        );
        println!(
            "  Range ({} … {}):     {:>9} … {:>9}",
            Paint::cyan("min"),
            Paint::magenta("max"),
            Paint::cyan(&min),
            Paint::magenta(&max),
        );
    }
}

fn print_run_summary(parts: &[Run]) {
    for (i, part) in parts.iter().enumerate() {
        let Run {
            name,
            result,
            elapsed,
        } = part;
        if i != 0 {
            println!();
        }
        let width = 46_usize.saturating_sub(name.chars().count() + 2);
        println!(
            "{}: {:>width$}\n{}",
            Paint::cyan(&name).bold(),
            Paint::fixed(&format!("({})", human::Time::new(*elapsed)), 245),
            Paint::new(&result).bold(),
            width = width,
        );
    }
}
