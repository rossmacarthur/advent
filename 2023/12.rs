use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    input
        .lines()
        .map(|condition| {
            let (springs, groups) = condition.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '?' => Spring::Unknown,
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    _ => panic!("unexpected character `{}`", c),
                })
                .collect();
            let groups = groups
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            (springs, groups)
        })
        .collect()
}

fn default_input() -> Vec<(Vec<Spring>, Vec<usize>)> {
    parse_input(include_input!(2023 / 12))
}

#[derive(Clone, Copy)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(usize, usize, usize);

impl State {
    /// Returns the starting state
    const fn start() -> Self {
        Self(0, 0, 0)
    }

    /// Returns the next state when current spring is operational
    const fn next(self) -> Self {
        let Self(s, g, _) = self;
        Self(s + 1, g, 0)
    }

    /// Returns the next state when the current spring is damaged
    const fn next_in_group(self) -> Self {
        let Self(s, g, c) = self;
        Self(s + 1, g, c + 1)
    }

    /// Returns the next state when the current group is complete
    const fn next_group(self) -> Self {
        let Self(s, g, _) = self;
        Self(s + 1, g + 1, 0)
    }
}

/// Returns the number of arrangements of springs that satisfy the given damaged
/// group specification. Count is the number of damaged springs already counted
/// in the first group.
fn arrangements_impl(
    cache: &mut HashMap<State, usize>,
    springs: &[Spring],
    groups: &[usize],
    state: State,
) -> usize {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    // s = current spring
    // g = current group
    // c = already counted springs in group
    let State(s, g, c) = state;

    // Base case
    if springs[s..].is_empty() {
        return match groups[g..] {
            [exp] if c == exp => 1, // damaged group is complete
            [] if c == 0 => 1,      // no damaged groups
            _ => 0,
        };
    }

    // The next possible states of the next spring (either damaged or not)
    let nexts: &[bool] = match springs[s] {
        Spring::Unknown => &[true, false],
        Spring::Operational => &[false],
        Spring::Damaged => &[true],
    };

    let mut result = 0;

    for &is_damaged in nexts {
        if is_damaged {
            // If the next spring is damaged then we should count it to the
            // current group.
            match groups[g..].first() {
                Some(&exp) if c < exp => {
                    result += arrangements_impl(cache, springs, groups, state.next_in_group());
                }
                _ => {}
            }
        } else {
            // If the next spring is operational then might be able to
            // "complete" a damaged group.
            match groups[g..].first() {
                _ if c == 0 => {
                    // If the count is zero it's fine we're just between groups.
                    result += arrangements_impl(cache, springs, groups, state.next());
                }
                Some(&exp) if c == exp => {
                    // The damaged group is complete, so we can recurse starting
                    // on the next group.
                    result += arrangements_impl(cache, springs, groups, state.next_group());
                }
                _ => {}
            }
        }
    }

    cache.insert(state, result);

    result
}

fn arrangements((springs, groups): (Vec<Spring>, Vec<usize>)) -> usize {
    let mut cache = HashMap::new();
    arrangements_impl(&mut cache, &springs, &groups, State::start())
}

fn unfold((mut springs, mut groups): (Vec<Spring>, Vec<usize>)) -> (Vec<Spring>, Vec<usize>) {
    let springs = {
        let n = springs.len();
        springs.reserve(n * 4 + 4);
        for _ in 0..4 {
            springs.push(Spring::Unknown);
            springs.extend_from_within(..n);
        }
        springs
    };
    let groups = {
        let n = groups.len();
        groups.reserve(n * 4);
        for _ in 0..4 {
            groups.extend_from_within(..n);
        }
        groups
    };
    (springs, groups)
}

fn part1(conditions: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    conditions.into_iter().map(arrangements).sum()
}

fn part2(conditions: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    conditions.into_iter().map(unfold).map(arrangements).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    );
    assert_eq!(part1(input.clone()), 21);
    assert_eq!(part2(input), 525152);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7173);
    assert_eq!(part2(input), 29826669191291);
}
