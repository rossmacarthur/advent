use std::process;

use clap::{AppSettings, Clap};

////////////////////////////////////////////////////////////////////////////////
// Days
////////////////////////////////////////////////////////////////////////////////

fn day01() {
    use advent::day01::*;

    let input = default_input();

    println!("Part 1:");
    let (a, b) = solve_sum_two(input.clone()).unwrap();
    println!("  The numbers are: {}, {}", a, b);
    println!("  Multiplied together: {}", a * b);

    println!("Part 2:");
    let (a, b, c) = solve_sum_three(input).unwrap();
    println!("  The numbers are: {}, {}, {}", a, b, c);
    println!("  Multiplied together: {}", a * b * c);
}

fn day02() {
    use advent::day02::*;

    let input = default_input();

    println!("Part 1:");
    let valid = valid_with_count_policy(&input);
    println!("  The number of valid passwords is: {}", valid);

    println!("Part 2:");
    let valid = valid_with_position_policy(&input);
    println!("  The number of valid passwords is: {}", valid);
}

fn day03() {
    use advent::day03::*;

    let input = default_input();

    println!("Part 1:");
    let trees = count_trees_single_slope(&input);
    println!("  The number of trees is {}", trees);

    println!("Part 2:");
    let trees = count_trees_many_slopes(&input);
    println!("  Each number of trees multiplied together is {}", trees);
}

////////////////////////////////////////////////////////////////////////////////
// Command line interface
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clap)]
#[clap(global_setting = AppSettings::DisableVersion)]
struct Opt {
    #[clap(name = "DAY")]
    day: Option<u32>,
}

fn main() {
    let Opt { day } = Opt::parse();
    match day {
        Some(1) => day01(),
        Some(2) => day02(),
        Some(3) | None => day03(),
        Some(d) => {
            eprintln!("Error: unknown day `{}`", d);
            process::exit(2);
        }
    }
}
