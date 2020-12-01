use aoc::day01::*;

fn main() {
    let input = input();

    println!("Part 1:");
    let (a, b) = solve_sum_two(&input).unwrap();
    println!("  The numbers are: {}, {}", a, b);
    println!("  Multiplied together: {}", a * b);

    println!("Part 2:");
    let (a, b, c) = solve_sum_three(&input).unwrap();
    println!("  The numbers are: {}, {}, {}", a, b, c);
    println!("  Multiplied together: {}", a * b * c);
}
