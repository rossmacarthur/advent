fn default_input() -> &'static str {
    "047801"
}

fn solve<F>(cond: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> bool,
{
    let mut scores = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    while cond(&scores) {
        let sum = scores[i] + scores[j];
        if sum >= 10 {
            scores.push(sum / 10);
            scores.push(sum % 10);
        } else {
            scores.push(sum as u8);
        }
        i = (i + scores[i] as usize + 1) % scores.len();
        j = (j + scores[j] as usize + 1) % scores.len();
    }
    scores
}

fn part1(input: &str) -> usize {
    // Simulate the loop until we have n+10 scores.
    let n = input.parse().unwrap();
    let scores = solve(|scores| scores.len() < n + 10);
    scores[n..n + 10]
        .iter()
        .fold(0, |acc, &i| acc * 10 + i as usize)
}

fn part2(input: &str) -> usize {
    // Simulate the loop until we find the score sequence, we need
    // to consider the case where it is not at the end in the case
    // where the final value was the result of a sum > 10.
    let needle: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    let scores = solve(|scores| {
        !(scores.ends_with(&needle) || scores[..scores.len() - 1].ends_with(&needle))
    });

    let n = needle.len();
    if scores.ends_with(&needle) {
        scores.len() - n
    } else {
        scores.len() - n - 1
    }
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    assert_eq!(part1("5"), 124515891);
    assert_eq!(part1("18"), 9251071085);
    assert_eq!(part1("2018"), 5941429882);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1342316410);
    assert_eq!(part2(input), 20235230);
}
