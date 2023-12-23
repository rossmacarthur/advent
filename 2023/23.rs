use advent::prelude::*;

fn get_start_and_end(map: &HashMap<Vector2, Tile>) -> (Vector2, Vector2) {
    let find = |y: i64| {
        map.iter()
            .find_map(|(&p, &t)| (p.y == y && t == Tile::Path).some(p))
            .unwrap()
    };
    let (min_y, max_y) = map.keys().map(|p| p.y).min_max().unwrap();
    (find(min_y), find(max_y))
}

fn parse_input(input: &str) -> (HashMap<Vector2, Tile>, Vector2, Vector2) {
    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' => Tile::Slope(UP),
        '>' => Tile::Slope(RIGHT),
        'v' => Tile::Slope(DOWN),
        '<' => Tile::Slope(LEFT),
        _ => panic!("unexpected character `{}`", c),
    });
    let (start, end) = get_start_and_end(&map);
    (map, start, end)
}

fn default_input() -> (HashMap<Vector2, Tile>, Vector2, Vector2) {
    parse_input(include_input!(2023 / 23))
}

/// A directed graph of positions to adjacent positions and the distance to move
/// between them.
type Graph = HashMap<Vector2, Vec<(Vector2, usize)>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Vector2),
}
const UP: Vector2 = vector![0, -1];
const DOWN: Vector2 = vector![0, 1];
const RIGHT: Vector2 = vector![1, 0];
const LEFT: Vector2 = vector![-1, 0];
const DIRECTIONS: [Vector2; 4] = [UP, DOWN, RIGHT, LEFT];

fn to_graph(map: &HashMap<Vector2, Tile>, part2: bool) -> Graph {
    let mut graph = HashMap::new();
    for (&p, &t) in map {
        if t == Tile::Forest {
            continue;
        }
        let ns: Vec<_> = DIRECTIONS
            .into_iter()
            .filter_map(|d| {
                let next = p + d;
                match map.get(&next) {
                    Some(&Tile::Path) => Some((next, 1)),
                    Some(&Tile::Slope(slope)) if part2 || d == slope => Some((next, 1)),
                    _ => None,
                }
            })
            .collect();
        graph.insert(p, ns);
    }

    if part2 {
        prune_graph(&mut graph);
    }

    graph
}

// Iteratively prune the graph, everything with two neighbours can be reduced to
// an edge between those two neighbours. This assumes that for every edge a -> b
// there is also an edge b -> a. For example
//
//        d1      d2
//     a ----> b ----> c
//
//  becomes
//
//          d1 + d2
//     a -------------> c
//
fn prune_graph(graph: &mut Graph) {
    let start = graph.iter().filter_map(|(&p, ns)| (ns.len() == 2).some(p));
    let mut q = VecDeque::from_iter(start);
    while let Some(b) = q.pop_front() {
        let Some(&[(a, d1), (c, d2)]) = graph.get(&b).map(|ns| ns.as_slice()) else {
            continue;
        };

        // Remove b and add c in a's neighbours, add c to the queue
        graph.entry(a).and_modify(|ns| {
            ns.retain(|&(n, _)| n != b);
            ns.push((c, d1 + d2));
            q.push_back(a);
        });

        // Remove b and add a in c's neighbours, add b to the queue
        graph.entry(c).and_modify(|ns| {
            ns.retain(|&(n, _)| n != b);
            ns.push((a, d1 + d2));
            q.push_back(c);
        });

        // Finally we can remove b from the graph
        graph.remove(&b);
    }
}

fn longest(graph: Graph, start: Vector2, end: Vector2) -> usize {
    let mut visited = HashSet::from_iter([start]);
    longest_impl(&graph, &mut visited, start, end, 0)
}

fn longest_impl(
    graph: &Graph,
    visited: &mut HashSet<Vector2>,
    pos: Vector2,
    end: Vector2,
    steps: usize,
) -> usize {
    if pos == end {
        return steps;
    }
    let mut len = 0;
    for &(next, step) in &graph[&pos] {
        if !visited.insert(next) {
            continue;
        }
        len = max(len, longest_impl(graph, visited, next, end, steps + step));
        visited.remove(&next);
    }
    len
}

fn part1((map, start, end): (HashMap<Vector2, Tile>, Vector2, Vector2)) -> usize {
    longest(to_graph(&map, false), start, end)
}

fn part2((map, start, end): (HashMap<Vector2, Tile>, Vector2, Vector2)) -> usize {
    longest(to_graph(&map, true), start, end)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
    );
    assert_eq!(part1(input.clone()), 94);
    assert_eq!(part2(input), 154);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1930);
    assert_eq!(part2(input), 6230);
}
