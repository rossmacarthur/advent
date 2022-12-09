use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Segment> {
    input
        .trim()
        .chars()
        .filter_map(|c| {
            let r = match c {
                'N' => Segment::Navigate(vector![0, 1]),
                'S' => Segment::Navigate(vector![0, -1]),
                'E' => Segment::Navigate(vector![-1, 0]),
                'W' => Segment::Navigate(vector![1, 0]),
                '(' => Segment::BeginBranch,
                '|' => Segment::NextOption,
                ')' => Segment::EndBranch,
                '^' | '$' => return None,
                c => panic!("unexpected character `{}`", c),
            };
            Some(r)
        })
        .collect()
}

fn default_input() -> Vec<Segment> {
    parse_input(include_str!("input/20.txt"))
}

#[derive(Clone)]
enum Segment {
    BeginBranch,
    NextOption,
    EndBranch,
    Navigate(Vector2),
}

fn solve(regex: Vec<Segment>) -> HashMap<Vector2, usize> {
    let mut distances = HashMap::new();
    let mut options = Vec::new();
    let mut state = (vector![0, 0], 0);
    for s in regex {
        match s {
            Segment::BeginBranch => {
                options.push(state);
            }
            Segment::NextOption => {
                state = *options.last().unwrap();
            }
            Segment::EndBranch => {
                state = options.pop().unwrap();
            }
            Segment::Navigate(d) => {
                let (pos, dist) = &mut state;
                *pos += d;
                *dist += 1;
                distances
                    .entry(*pos)
                    .and_modify(|d| {
                        if dist < d {
                            *d = *dist;
                        }
                    })
                    .or_insert(*dist);
            }
        }
    }
    distances
}

fn part1(regex: Vec<Segment>) -> usize {
    let distances = solve(regex);
    *distances.values().max().unwrap()
}

fn part2(regex: Vec<Segment>) -> usize {
    let distances = solve(regex);
    distances.values().filter(|&&d| d >= 1000).count()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input("^WNE$");
    assert_eq!(part1(input), 3);
}

#[test]
fn example2() {
    let input = parse_input("^ENWWW(NEEE|SSE(EE|N))$");
    assert_eq!(part1(input), 10);
}

#[test]
fn example3() {
    let input = parse_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
    assert_eq!(part1(input), 18);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4406);
    assert_eq!(part2(input), 8468);
}
