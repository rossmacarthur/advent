use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vector3> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .next_array()
                .unwrap()
        })
        .map(Vector3::from)
        .collect()
}

fn default_input() -> Vec<Vector3> {
    parse_input(include_input!(2025 / 08))
}

/// Returns an iterator over all combinations of two junction boxes, ordered
/// by Euclidean distance.
fn combinations(boxes: &[Vector3]) -> impl Iterator<Item = [usize; 2]> + '_ {
    (0..boxes.len())
        .array_combinations()
        .sorted_by_cached_key(|&[i, j]| {
            let d = boxes[i] - boxes[j];
            d.x * d.x + d.y * d.y + d.z * d.z
        })
}

/// Models the circuits using a Union-Find algorithm, the `parent` vector stores
/// the parent of each element. If an element is its own parent, it is the root
/// of its circuit. We start with each element in its own circuit.
///
/// <https://en.wikipedia.org/wiki/Disjoint-set_data_structure>
struct Circuits {
    parent: Vec<usize>,
}

impl Circuits {
    fn new(n: usize) -> Self {
        let parent = Vec::from_iter(0..n);
        Self { parent }
    }

    /// Finds the root of the circuit containing `k`
    fn root(&self, mut k: usize) -> usize {
        while self.parent[k] != k {
            k = self.parent[k];
        }
        k
    }

    /// Connects the circuits containing `i` and `j`.
    ///
    /// Returns `None` if they were already connected, otherwise returns the
    /// roots of the two circuits that were just connected. The second root is
    /// the root of the circuit that was merged into the first circuit.
    fn connect(&mut self, i: usize, j: usize) -> Option<(usize, usize)> {
        let a = self.root(i); // root of the circuit containing i
        let b = self.root(j); // root of the circuit containing j
        if a == b {
            return None;
        }
        self.parent[b] = a;
        Some((a, b))
    }
}

fn part1(boxes: Vec<Vector3>, connections: usize) -> usize {
    let n = boxes.len();

    let mut circuits = Circuits::new(n);
    for [i, j] in combinations(&boxes).take(connections) {
        circuits.connect(i, j);
    }

    // Now count the number of elements in each circuit, then take the product
    // of the sizes of the three largest circuits
    let lengths = (0..n)
        .map(|i| circuits.root(i))
        .fold(HashMap::new(), |mut acc, root| {
            *acc.entry(root).or_insert(0) += 1;
            acc
        });
    lengths.values().sorted_unstable().rev().take(3).product()
}

fn part2(boxes: Vec<Vector3>) -> i64 {
    let n = boxes.len();

    let mut circuits = Circuits::new(n);
    let mut size = vec![1; n]; // the size of each circuit, indexed by the root
    for [i, j] in combinations(&boxes) {
        if let Some((a, b)) = circuits.connect(i, j) {
            // Update the size of the new circuit
            size[a] += size[b];
            // Check if the circuit now contains all junction boxes
            if size[a] == n {
                return boxes[i].x * boxes[j].x;
            }
        }
    }
    unreachable!("all junction boxes should be connected")
}

fn main() {
    let solution = advent::new(default_input)
        .part(|input| part1(input, 1000))
        .part(part2)
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    );
    assert_eq!(part1(input.clone(), 10), 40);
    assert_eq!(part2(input), 25272);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone(), 1000), 98696);
    assert_eq!(part2(input), 2245203960);
}
