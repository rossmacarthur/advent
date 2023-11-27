use advent::prelude::*;

fn default_input() -> &'static [usize] {
    &[2, 20, 0, 4, 1, 17]
}

/// Finds the nth term of the Van Eck sequence that continues from the given
/// starting terms.
///
/// The method used here stores the lower numbers in a Vec to avoid expensive
/// hashing. Less frequent higher numbers are stored in a HashMap.
fn nth(seq: &[usize], n: usize) -> usize {
    // Numbers less than this will be stored in `low`, numbers higher will be
    // stored in `high`.
    let bounds = n / 10;

    let mut low = vec![0; bounds];
    let mut high = HashMap::with_capacity(bounds / 2);

    // Add the initial terms.
    for (turn, &n) in seq.iter().enumerate() {
        low[n] = turn + 1;
    }

    let mut curr = 0;
    for turn in (seq.len() + 1)..n {
        if curr < bounds {
            let prev = &mut low[curr];
            curr = if *prev == 0 { 0 } else { turn - *prev };
            *prev = turn;
        } else {
            curr = high.insert(curr, turn).map(|prev| turn - prev).unwrap_or(0)
        }
    }
    curr
}

fn part1(seq: &[usize]) -> usize {
    nth(seq, 2020)
}

fn part2(seq: &[usize]) -> usize {
    nth(seq, 30000000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    for (input, p1, p2) in [
        (&[0, 3, 6], 436, 175594),
        (&[1, 3, 2], 1, 2578),
        (&[2, 1, 3], 10, 3544142),
        (&[1, 2, 3], 27, 261214),
        (&[2, 3, 1], 78, 6895259),
        (&[3, 2, 1], 438, 18),
        (&[3, 1, 2], 1836, 362),
    ] {
        assert_eq!(part1(input), p1);
        assert_eq!(part2(input), p2);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 758);
    assert_eq!(part2(input), 814);
}
