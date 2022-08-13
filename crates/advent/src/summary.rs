use peter::Stylize;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum Summary {
    #[cfg_attr(feature = "json", serde(rename = "benches"))]
    Bench(Vec<Bench>),
    #[cfg_attr(feature = "json", serde(rename = "runs"))]
    Run(Vec<Run>),
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Bench {
    pub name: String,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub stats: Stats,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Run {
    pub name: String,
    pub result: String,
    pub elapsed: f64,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Stats {
    pub samples: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
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
        serde_json::to_writer(std::io::stdout(), self).unwrap();
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
            name.bold(),
            human::Samples::new(stats.samples).fixed(245),
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
            name.bold().cyan(),
            format!("({})", human::Time::new(*elapsed)).fixed(245),
            result.bold(),
            width = width,
        );
    }
}