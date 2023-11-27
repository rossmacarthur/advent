use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    regex!("Step ([A-Z]) must be finished before step ([A-Z]) can begin")
        .captures_iter(input)
        .map(|caps| {
            let a = caps[1].bytes().exactly_one().unwrap();
            let b = caps[2].bytes().exactly_one().unwrap();
            (a, b)
        })
        .collect()
}

fn default_input() -> Vec<(u8, u8)> {
    parse_input(include_str!("input/07.txt"))
}

fn solve(instrs: Vec<(u8, u8)>, workers: usize, dt: usize) -> (String, usize) {
    // The time taken to complete a step.
    let dt = |step: u8| 1 + dt + usize::from(step - b'A');

    // Create a map a -> {b, ..} where each value, b, can be completed after
    // each key, a.
    let mut after: HashMap<_, Vec<_>> = HashMap::new();
    for &(a, b) in &instrs {
        after.entry(a).or_default().push(b);
    }

    // Create a map b -> n where each value, n, is the number of steps that must
    // be completed before each key, b.
    let mut before: HashMap<_, usize> = HashMap::new();
    for &(_, b) in &instrs {
        *before.entry(b).or_default() += 1;
    }

    // A queue of available steps to work on. We initialize this using all steps
    // that don't have any dependencies.
    let mut steps: BinaryHeap<_> = after
        .keys()
        .copied()
        .filter(|step| before.get(step).is_none())
        .map(Reverse)
        .collect();

    // Contains the steps currently being processed by the workers. The front of
    // the queue will be the step that will be completed the soonest.
    let mut q = BinaryHeap::new();

    // Tracks the order the steps were completed (Part 1).
    let mut order = Vec::new();

    // Tracks the total amount of time taken to complete the steps (Part 2).
    let mut time = 0;

    while let Some(Reverse((t, step))) = {
        // Give the workers some steps to process
        while q.len() < workers && !steps.is_empty() {
            let Reverse(step) = steps.pop().unwrap();
            q.push(Reverse((time + dt(step), step)));
        }
        q.pop()
    } {
        order.push(step);
        time = t;
        // Once a step is finished, add all the steps to the queue that can now
        // be completed.
        if let Some(after) = after.get(&step) {
            for s in after {
                let deps = before.get_mut(s).unwrap();
                *deps -= 1;
                if *deps == 0 {
                    steps.push(Reverse(*s));
                }
            }
        }
    }

    (String::from_utf8(order).unwrap(), time)
}

fn part1(instrs: Vec<(u8, u8)>) -> String {
    solve(instrs, 1, 0).0
}

fn part2(instrs: Vec<(u8, u8)>, workers: usize, dt: usize) -> usize {
    solve(instrs, workers, dt).1
}

fn main() {
    let solution = advent::new(default_input)
        .part(part1)
        .part(|i| part2(i, 5, 60))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
    );
    assert_eq!(part1(input.clone()), "CABDFE");
    assert_eq!(part2(input, 2, 0), 15);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "CFMNLOAHRKPTWBJSYZVGUQXIDE");
    assert_eq!(part2(input, 5, 60), 971);
}
