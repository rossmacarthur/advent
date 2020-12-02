use std::env;
use std::process;

fn day01() {
    use aoc::day01::*;

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
    use aoc::day02::*;

    let input = default_input();

    println!("Part 1:");
    let valid = valid_with_count_policy(&input);
    println!("  The number of valid passwords is: {}", valid);

    println!("Part 2:");
    let valid = valid_with_position_policy(&input);
    println!("  The number of valid passwords is: {}", valid);
}

fn main() {
    let owned_args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = owned_args.iter().map(|s| s.as_str()).collect();
    let day: u32 = match args.as_slice() {
        [a] => a.parse().unwrap(),
        _ => {
            eprintln!("expected a single 'day' argument, got: {:?}", args);
            process::exit(1);
        }
    };
    match day {
        1 => day01(),
        2 => day02(),
        d => {
            eprintln!("unknown day: {}", d);
            process::exit(2);
        }
    }
}
