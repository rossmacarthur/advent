use aoc::day02::*;

fn main() {
    let input = default_input();

    println!("Part 1:");
    let valid = valid_with_count_policy(&input);
    println!("  The number of valid passwords is: {}", valid);

    println!("Part 2:");
    let valid = valid_with_position_policy(&input);
    println!("  The number of valid passwords is: {}", valid);
}
