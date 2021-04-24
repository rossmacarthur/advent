use std::process;

use clap::{AppSettings, Clap};

macro_rules! run {
    ($path:path) => {{
        use $path::*;
        let mut run = advent::start();
        let input = run.time("Parse input", default_input());
        run.result("Part 1", part1(&input));
        run.result("Part 2", part2(&input));
        run.finish();
    }};
}

#[derive(Debug, Clap)]
#[clap(global_setting = AppSettings::DisableVersion)]
struct Opt {
    #[clap(long, name = "YEAR")]
    year: u32,

    #[clap(long, name = "DAY")]
    day: Option<u32>,
}

fn main() {
    let Opt { year, day } = Opt::parse();

    match (year, day) {
        (2019, Some(1)) => run!(advent_2019::day01),
        (2019, Some(2)) => run!(advent_2019::day02),
        (2019, Some(3)) => run!(advent_2019::day03),
        (2019, Some(4)) => run!(advent_2019::day04),
        (2019, Some(5)) => run!(advent_2019::day05),
        (2019, Some(6)) => run!(advent_2019::day06),
        (2019, Some(7)) => run!(advent_2019::day07),
        (2019, Some(8)) => run!(advent_2019::day08),
        (2019, Some(9)) => run!(advent_2019::day09),
        (2019, Some(10)) => run!(advent_2019::day10),
        (2019, Some(11)) => run!(advent_2019::day11),
        (2019, Some(12)) | (2019, None) => run!(advent_2019::day12),

        (2020, Some(1)) => run!(advent_2020::day01),
        (2020, Some(2)) => run!(advent_2020::day02),
        (2020, Some(3)) => run!(advent_2020::day03),
        (2020, Some(4)) => run!(advent_2020::day04),
        (2020, Some(5)) => run!(advent_2020::day05),
        (2020, Some(6)) => run!(advent_2020::day06),
        (2020, Some(7)) => run!(advent_2020::day07),
        (2020, Some(8)) => run!(advent_2020::day08),
        (2020, Some(9)) => run!(advent_2020::day09),
        (2020, Some(10)) => run!(advent_2020::day10),
        (2020, Some(11)) => run!(advent_2020::day11),
        (2020, Some(12)) => run!(advent_2020::day12),
        (2020, Some(14)) => run!(advent_2020::day14),
        (2020, Some(15)) => run!(advent_2020::day15),
        (2020, Some(16)) => run!(advent_2020::day16),
        (2020, Some(17)) => run!(advent_2020::day17),
        (2020, Some(18)) => run!(advent_2020::day18),
        (2020, Some(19)) => run!(advent_2020::day19),
        (2020, Some(21)) => run!(advent_2020::day21),
        (2020, Some(22)) => run!(advent_2020::day22),
        (2020, Some(23)) => run!(advent_2020::day23),
        (2020, Some(24)) => run!(advent_2020::day24),
        (2020, Some(25)) | (2020, None) => run!(advent_2020::day25),

        (year, day) => {
            eprintln!("Error: unknown day `{:?}` for year `{:?}`", day, year);
            process::exit(2);
        }
    }
}
