const INPUT: &'static str = include_str!("input.txt");

fn solve_2(numbers: &[u32]) -> Option<(u32, u32)> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            if a + b == 2020 {
                return Some((a, b));
            }
        }
    }
    None
}

fn solve_3(numbers: &[u32]) -> Option<(u32, u32, u32)> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                let a = numbers[i];
                let b = numbers[j];
                let c = numbers[k];
                if a + b + c == 2020 {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

fn main() {
    let report: Vec<u32> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    println!("Part 1:");
    let (a, b) = solve_2(&report).unwrap();
    println!("  The numbers are: {}, {}", a, b);
    println!("  Multiplied together: {}", a * b);

    println!("Part 2:");
    let (a, b, c) = solve_3(&report).unwrap();
    println!("  The numbers are: {}, {}, {}", a, b, c);
    println!("  Multiplied together: {}", a * b * c);
}
