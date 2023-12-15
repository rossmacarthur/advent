use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vector4> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_array()
        })
        .map(Vector4::from)
        .collect()
}

fn default_input() -> Vec<Vector4> {
    parse_input(include_input!(2018 / 25))
}

fn part1(points: Vec<Vector4>) -> usize {
    // Implements a Union-Find algorithm, this vector stores the parent of each
    // element. If an element is its own parent, it is the root of its set.
    let mut parent = Vec::from_iter(0..points.len());

    // Find the root of a set that an element belongs to, this function also
    // compresses the path to the root.
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = find(parent, parent[x]);
            x = parent[x];
        }
        x
    }

    // Compare every point to every other point...
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i) {
            // If the points are within 3 units of each other, join their sets
            if (p1 - p2).l1_norm() <= 3 {
                let root = find(&mut parent, j);
                parent[root] = parent[i];
            }
        }
    }

    (0..points.len())
        .map(|i| find(&mut parent, i))
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0",
    );
    assert_eq!(part1(input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
    );
    assert_eq!(part1(input), 4);
}

#[test]
fn example3() {
    let input = parse_input(
        "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
    );
    assert_eq!(part1(input), 3);
}

#[test]
fn example4() {
    let input = parse_input(
        "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
    );
    assert_eq!(part1(input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 396);
}
