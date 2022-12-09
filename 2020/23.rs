use advent::prelude::*;

fn parse_input(s: &str) -> Vec<usize> {
    s.bytes().map(|b| usize::from(b - b'0')).collect()
}

fn default_input() -> Vec<usize> {
    parse_input("716892543")
}

fn play(cups: &[usize], moves: usize, n: Option<usize>) -> Vec<usize> {
    let n = n.unwrap_or(cups.len());

    let start = cups[0];

    // Construct a list such that each value used as an index refers to the next
    // value in the circle.
    let mut circle = {
        let mut circle = vec![0; n + 1];
        let mut last = start;
        for &c in &cups[1..] {
            circle[last] = c;
            last = c;
        }
        for c in (cups.len() + 1)..n + 1 {
            circle[last] = c;
            last = c;
        }
        circle[last] = start;
        circle
    };

    let mut curr = start;
    for _ in 0..moves {
        // Pick up the next three cups.
        let a = circle[curr];
        let b = circle[a];
        let c = circle[b];

        // Now find the destination cup.
        let dest = {
            let next = |d| if d > 1 { d - 1 } else { n };
            let mut d = next(curr);
            while [a, b, c].contains(&d) {
                d = next(d);
            }
            d
        };

        // Place the three cups down after the destination cup.
        {
            let tmp = circle[curr];
            circle[curr] = circle[c];
            circle[c] = circle[dest];
            circle[dest] = tmp;
        }

        // Finally select the next current cup.
        curr = circle[curr];
    }

    circle
}

fn iter(circle: &[usize]) -> impl Iterator<Item = usize> + '_ {
    iter::successors(Some(1), |&c| Some(circle[c]))
}

fn part1(cups: Vec<usize>) -> String {
    let circle = play(&cups, 100, None);
    let result = iter(&circle)
        .take(cups.len())
        .skip(1)
        .map(|c| b'0' + (c as u8))
        .collect();
    String::from_utf8(result).unwrap()
}

fn part2(cups: Vec<usize>) -> usize {
    let circle = play(&cups, 10_000_000, Some(1_000_000));
    iter(&circle).skip(1).take(2).product()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("389125467");
    assert_eq!(part1(input.clone()), "67384529");
    assert_eq!(part2(input), 149245887792);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "49725386");
    assert_eq!(part2(input), 538935646702);
}
