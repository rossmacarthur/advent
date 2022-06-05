use std::io;

use peter::Stylize;

use crate::human;
use crate::types::*;

pub fn print_summary<O: io::Write>(buf: O, summary: &Summary) -> io::Result<()> {
    match summary {
        Summary::Bench(parts) => print_bench_summary(buf, parts),
        Summary::Run(parts) => print_run_summary(buf, parts),
    }
}

fn print_bench_summary<O: io::Write>(mut buf: O, parts: &[Bench]) -> io::Result<()> {
    for (i, part) in parts.iter().enumerate() {
        let Bench { name, stats } = part;
        if i != 0 {
            writeln!(buf)?;
        }
        writeln!(
            buf,
            "{}{:>width$}",
            name.bold(),
            human::Samples::new(stats.len).fixed(245),
            width = 46 - name.chars().count(),
        )?;
        let mean = human::Time::new(stats.mean);
        let std_dev = human::Time::with_scale(stats.std_dev, mean.scale());
        let min = human::Time::with_scale(stats.min, mean.scale());
        let max = human::Time::with_scale(stats.max, mean.scale());
        writeln!(
            buf,
            "  Time ({} ± {}):       {:>9} ± {:>9}",
            "mean".green().bold(),
            "σ".green(),
            mean.green().bold(),
            std_dev.green(),
        )?;
        writeln!(
            buf,
            "  Range ({} … {}):     {:>9} … {:>9}",
            "min".cyan(),
            "max".magenta(),
            min.cyan(),
            max.magenta(),
        )?;
    }
    Ok(())
}

fn print_run_summary<O: io::Write>(mut buf: O, parts: &[Run]) -> io::Result<()> {
    for (i, part) in parts.iter().enumerate() {
        let Run {
            name,
            result,
            elapsed,
        } = part;
        if i != 0 {
            writeln!(buf)?;
        }
        let width = 46_usize.saturating_sub(name.chars().count() + 2);
        writeln!(
            buf,
            "{}: {:>width$}\n{}",
            name.bold().cyan(),
            format!("({})", human::Time::new(*elapsed)).fixed(245),
            result.bold(),
            width = width,
        )?;
    }
    Ok(())
}
