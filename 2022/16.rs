use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Valve> {
    let re = regex!(
        r"Valve (?P<name>[A-Z]+) has flow rate=(?P<fr>\d+); tunnels? leads? to valves? (?P<leads>.*)"
    );

    let specs: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let name = caps.name("name").unwrap().as_str();
            let leads_to: Vec<_> = caps.name("leads").unwrap().as_str().split(", ").collect();
            let flow_rate = caps.name("fr").unwrap().as_str().parse().unwrap();
            (name, leads_to, flow_rate)
        })
        .collect();

    // Convert each valve to a bit so that we can use bitmasks to represent
    // multiple valves. We do this by simply sorting the valves by name and
    // using the sorted index as the bit. For example: AA will be a number with
    // bit 0 set, BB will be a number with bit 1 set etc.
    let ids: HashMap<&str, usize> = specs
        .iter()
        .map(|(name, _, _)| name)
        .sorted()
        .enumerate()
        .map(|(i, &v)| (v, 1 << i))
        .collect();

    specs
        .into_iter()
        .map(|(name, leads_to, flow_rate)| {
            let id = ids[name];
            let leads_to = leads_to.into_iter().map(|n| ids[n]).collect();
            Valve {
                id,
                flow_rate,
                leads_to,
            }
        })
        .collect()
}

fn default_input() -> Vec<Valve> {
    parse_input(include_str!("input/16.txt"))
}

type Graph = HashMap<usize, HashMap<usize, i64>>;

const AA: usize = 0b1;
const NONE: usize = 0b0;

#[derive(Debug, Clone)]
struct Valve {
    id: usize,
    flow_rate: i64,
    leads_to: Vec<usize>,
}

// Use the Floyd-Warshall algorithm to create a graph of shortest path from
// every valve to every other valve.
//
// See https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
fn distances(valves: &[Valve]) -> Option<Graph> {
    let all: Vec<_> = valves.iter().map(|v| v.id).collect();

    let mut graph: HashMap<_, HashMap<_, _>> = all
        .iter()
        .map(|&v| (v, all.iter().map(|&v| (v, i64::MAX / 2)).collect()))
        .collect();

    for v in valves {
        let to = graph.get_mut(&v.id)?;
        *to.get_mut(&v.id)? = 0;
        for e in &v.leads_to {
            *to.get_mut(e)? = 1;
        }
    }

    for k in &all {
        for i in &all {
            for j in &all {
                let m = min(graph[i][j], graph[i][k] + graph[k][j]);
                *graph.get_mut(i)?.get_mut(j)? = m;
            }
        }
    }
    Some(graph)
}

fn solve(valves: Vec<Valve>, ttl: i64) -> HashMap<usize, i64> {
    // The full graph with shortest paths from every valve to every other valve
    let graph = distances(&valves).unwrap();

    // A map of valve to flow rate
    let rates: HashMap<_, _> = valves.iter().map(|v| (v.id, v.flow_rate)).collect();

    // All valves with a non-zero flow rate
    let important: Vec<_> = graph.keys().copied().filter(|v| rates[v] > 0).collect();

    // Brute force all possible routes to these valves and store the best amount
    // of total pressure released for any set of opened valves.
    let mut best = HashMap::new();

    let mut stack = VecDeque::from([(AA, NONE, ttl, 0)]);
    while let Some((v, opened, t, released)) = stack.pop_front() {
        // We only care about valves with a non-zero flow rate so only go to
        // those. Since we know the distances to all valves we can just
        // decrement the time by the distance to go there.
        for &n in &important {
            let dist = graph[&v][&n];
            // Only go to this valve if we haven't opened it yet and there is
            // enough time to get there.
            if opened & n == 0 && dist < t {
                let t = t - dist - 1;
                let released = released + rates[&n] * t;
                stack.push_back((n, opened | n, t, released));
            }
        }
        best.entry(opened)
            .and_modify(|r| {
                if released > *r {
                    *r = released;
                }
            })
            .or_insert(released);
    }

    best
}

fn part1(valves: Vec<Valve>) -> i64 {
    solve(valves, 30).into_values().max().unwrap()
}

fn part2(valves: Vec<Valve>) -> i64 {
    let best = solve(valves, 26);
    iproduct!(&best, &best)
        .filter_map(|((o1, r1), (o2, r2))| (o1 & o2 == 0).then(|| r1 + r2))
        .max()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
    );
    assert_eq!(part1(input.clone()), 1651);
    assert_eq!(part2(input), 1707);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1896);
    assert_eq!(part2(input), 2576);
}
