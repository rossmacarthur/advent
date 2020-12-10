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
    #[clap(name = "DAY")]
    day: Option<u32>,
}

fn main() {
    let Opt { day } = Opt::parse();

    match day {
        Some(1) => run!(advent::day01),
        Some(2) => run!(advent::day02),
        Some(3) => run!(advent::day03),
        Some(4) => run!(advent::day04),
        Some(5) => run!(advent::day05),
        Some(6) => run!(advent::day06),
        Some(7) => run!(advent::day07),
        Some(8) => run!(advent::day08),
        Some(9) => run!(advent::day09),
        Some(10) | None => run!(advent::day10),
        Some(d) => {
            eprintln!("Error: unknown day `{}`", d);
            process::exit(2);
        }
    }
}