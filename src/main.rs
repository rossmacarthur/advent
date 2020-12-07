use std::process;

use clap::{AppSettings, Clap};

macro_rules! run {
    ($path:path) => {{
        use $path::*;
        let input = default_input();
        let result = part1(&input);
        println!("Part 1: {:?}", result);
        let result = part2(&input);
        println!("Part 2: {:?}", result);
    }};
}

#[derive(Debug, Clap)]
#[clap(global_setting = AppSettings::DisableVersion)]
struct Opt {
    #[clap(long, short, name = "YEAR", default_value = "2020")]
    year: u32,

    #[clap(name = "DAY")]
    day: Option<u32>,
}

fn main() {
    let Opt { year, day } = Opt::parse();

    match year {
        2020 => match day {
            Some(1) => run!(advent::year2020::day01),
            Some(2) => run!(advent::year2020::day02),
            Some(3) => run!(advent::year2020::day03),
            Some(4) => run!(advent::year2020::day04),
            Some(5) => run!(advent::year2020::day05),
            Some(6) => run!(advent::year2020::day06),
            Some(7) | None => run!(advent::year2020::day07),
            Some(d) => {
                eprintln!("Error: unknown day `{}` for year `{}`", d, year);
                process::exit(2);
            }
        },
        year => {
            eprintln!("Error: unknown year `{}`", year);
            process::exit(2);
        }
    }
}
