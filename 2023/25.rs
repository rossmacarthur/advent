use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            (l, r.split_whitespace().collect())
        })
        .collect()
}

fn default_input() -> HashMap<&'static str, Vec<&'static str>> {
    parse_input(include_input!(2023 / 25))
}

fn kargers_algorithm(graph: &mut HashMap<String, Vec<String>>) -> usize {
    let mut rng = rand::thread_rng();
    while graph.len() > 2 {
        let edge = select_random_edge(graph, &mut rng);
        contract_edge(graph, edge.0, edge.1);
    }

    graph.values().next().unwrap().len()
}

fn select_random_edge<'a>(
    graph: &HashMap<String, Vec<String>>,
    rng: &mut impl Rng,
) -> (String, String) {
    let start_vertex = graph.keys().nth(rng.gen_range(0..graph.len())).unwrap();
    let end_vertex = &graph[start_vertex][rng.gen_range(0..graph[start_vertex].len())];
    (start_vertex.clone(), end_vertex.clone())
}

fn contract_edge(graph: &mut HashMap<String, Vec<String>>, v1: String, v2: String) {
    let v1_edges = graph.remove(&v1).unwrap();
    let v2_edges = graph.remove(&v2).unwrap();

    let new_vertex = format!("{}_{}", v1, v2);
    let mut new_edges = Vec::new();

    for edge in v1_edges.iter().chain(v2_edges.iter()) {
        if edge != &v1 && edge != &v2 {
            new_edges.push(edge.clone());
            graph.entry(edge.clone()).and_modify(|e| {
                for e_edge in e.iter_mut() {
                    if e_edge == &v1 || e_edge == &v2 {
                        *e_edge = new_vertex.clone();
                    }
                }
            });
        }
    }

    graph.insert(new_vertex, new_edges);
}

fn calculate_subgraph_sizes(graph: &HashMap<String, Vec<String>>) -> usize {
    let [left, right] = graph.keys().next_array().unwrap();
    left.split("_").count() * right.split("_").count()
}

fn part1(components: HashMap<&str, Vec<&str>>) -> usize {
    let mut g = {
        let mut g = components.clone();
        let reversed = components
            .into_iter()
            .flat_map(|(l, rs)| rs.into_iter().map(move |r| (r, l)));
        for (l, r) in reversed {
            g.entry(l).or_default().push(r);
        }
        g
    };

    let owned_g: HashMap<_, Vec<_>> = g
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.into_iter().map(|s| s.to_owned()).collect()))
        .collect();

    for _ in 0.. {
        let mut owned_g = owned_g.clone();
        let min_cut = kargers_algorithm(&mut owned_g);
        if min_cut == 3 {
            return calculate_subgraph_sizes(&owned_g);
        }
    }

    0
}

fn part2(input: HashMap<&str, Vec<&str>>) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
    );
    assert_eq!(part1(input), 54);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
