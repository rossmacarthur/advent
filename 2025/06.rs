use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 06)
}

/// Trims whitespace from a byte slice and converts it to an integer
fn atoi(b: impl AsRef<[u8]>) -> i64 {
    let s = str::from_utf8(b.as_ref()).expect("valid utf8");
    s.trim().parse().expect("expected integer")
}

/// Transpose a 2D collection
fn transpose<T: Copy>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let cols = rows.iter().map(Vec::len).max().unwrap_or(0);
    (0..cols)
        .map(|c| rows.iter().filter_map(|r| r.get(c).copied()).collect())
        .collect()
}

fn part1(input: &str) -> i64 {
    let input = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    transpose(input)
        .into_iter()
        .map(|mut col| -> i64 {
            match col.pop().expect("operator") {
                "+" => col.iter().map(atoi).sum(),
                "*" => col.iter().map(atoi).product(),
                op => panic!("unexpected operator `{op}`"),
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut input: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    iter::zip(
        input.pop().unwrap().into_iter().filter(|&b| b != b' '),
        transpose(input).split(|col| col.trim_ascii().is_empty()),
    )
    .map(|(op, nums)| -> i64 {
        match op {
            b'+' => nums.iter().map(atoi).sum(),
            b'*' => nums.iter().map(atoi).product(),
            op => panic!("unexpected operator `{op}`"),
        }
    })
    .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn example() {
    let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part1(input), 4277556);
    assert_eq!(part2(input), 3263827);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 4648618073226);
    assert_eq!(part2(input), 7329921182115);
}
