use advent::prelude::*;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> Graph<'_> {
    input
        .lines()
        .flat_map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            [(from, to), (to, from)]
        })
        .fold(HashMap::new(), |mut graph, (from, to)| {
            graph.entry(from).or_default().push(to);
            graph
        })
}

fn default_input() -> Graph<'static> {
    parse_input(include_str!("input/12.txt"))
}

fn solve(graph: &Graph<'_>, is_dup_allowed: bool) -> usize {
    let mut q = VecDeque::from([(vec!["end"], None)]);
    let mut paths = 0;
    while let Some((path, dup)) = q.pop_front() {
        for &cave in &graph[path.last().unwrap()] {
            match cave {
                "end" => continue,
                "start" => paths += 1,
                cave => {
                    let mut dup = dup;
                    if cave.chars().all(char::is_lowercase) && path.contains(&cave) {
                        if !is_dup_allowed || dup.is_some() {
                            continue;
                        }
                        dup = Some(cave);
                    };
                    let next = path.iter().cloned().chain([cave]).collect();
                    q.push_back((next, dup));
                }
            }
        }
    }
    paths
}

fn part1(graph: Graph<'_>) -> usize {
    solve(&graph, false)
}

fn part2(graph: Graph<'_>) -> usize {
    solve(&graph, true)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    );
    assert_eq!(part1(input.clone()), 10);
    assert_eq!(part2(input), 36);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    );
    assert_eq!(part1(input.clone()), 19);
    assert_eq!(part2(input), 103);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    );
    assert_eq!(part1(input.clone()), 226);
    assert_eq!(part2(input), 3509);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3761);
    assert_eq!(part2(input), 99138);
}
