use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (src, dst) = line.split_once(":").unwrap();
            (src, dst.split_whitespace().collect())
        })
        .collect()
}

fn default_input() -> HashMap<&'static str, Vec<&'static str>> {
    parse_input(include_input!(2025 / 11))
}

fn count_paths(graph: HashMap<&str, Vec<&str>>, part1: bool, start: &str) -> usize {
    fn f<'a>(
        graph: &HashMap<&'a str, Vec<&'a str>>,
        part1: bool,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        state: (&'a str, bool, bool),
    ) -> usize {
        let (device, fft, dac) = state;
        if device == "out" {
            return (part1 || (fft && dac)) as usize;
        }
        graph[device]
            .iter()
            .map(|&d| {
                let next = (d, fft || d == "fft", dac || d == "dac");
                match cache.get(&next) {
                    Some(&paths) => paths,
                    None => {
                        let paths = f(graph, part1, cache, next);
                        cache.insert(next, paths);
                        paths
                    }
                }
            })
            .sum()
    }

    let mut cache = HashMap::new();
    f(&graph, part1, &mut cache, (start, false, false))
}

fn part1(graph: HashMap<&str, Vec<&str>>) -> usize {
    count_paths(graph, true, "you")
}

fn part2(graph: HashMap<&str, Vec<&str>>) -> usize {
    count_paths(graph, false, "svr")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
    );
    assert_eq!(part1(input), 5);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    );
    assert_eq!(part2(input), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 448);
    assert_eq!(part2(input), 553204221431080);
}
