use advent::prelude::*;

fn parse_coord(s: &str) -> Vector3 {
    s.split(',').map(str::parse).map(Result::unwrap).collect()
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (m, n) = line.split_once('~').unwrap();
            Brick::new(parse_coord(m), parse_coord(n))
        })
        .collect()
}

fn default_input() -> Vec<Brick> {
    parse_input(include_input!(2023 / 22))
}

/// Represents a brick in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    m: Vector3,
    n: Vector3,
    d: usize,
}

impl Brick {
    /// Constructs a new brick from two points ensuring that the invariants are
    /// upheld.
    ///
    /// The invariants are:
    /// - only one dimension changes between the two points
    /// - the points are ordered by the changing dimension
    #[track_caller]
    fn new(m: Vector3, n: Vector3) -> Self {
        let d = {
            let mut it = (0..3).filter(|&d| m[d] != n[d]);
            let d = it.next().unwrap_or(0);
            assert!(it.next().is_none(), "only one dimension should change");
            d
        };
        assert!(
            m[d] <= n[d],
            "points should be ordered by the changing dimension"
        );
        Self { m, n, d }
    }

    /// Returns an iterator over all the points in the brick.
    fn iter(&self) -> impl Iterator<Item = Vector3> + '_ {
        let Self { m, n, d } = *self;
        (m[d]..=n[d]).map(move |vv| {
            let mut v = m;
            v[d] = vv;
            v
        })
    }

    /// Moves the brick down such that its minimum z value is `z`.
    fn fall(&mut self, z: i64) {
        let dz = z - self.min_z();
        self.m.z += dz;
        self.n.z += dz;
    }

    /// Returns the minimum z value of the brick.
    fn min_z(&self) -> i64 {
        min(self.m.z, self.n.z)
    }
}

type Graph = HashMap<usize, HashSet<usize>>;

/// Simulates the bricks falling and returns the "supported by" graph which is
/// is a map of brick to the set of bricks below it.
fn fall(mut bricks: Vec<Brick>) -> Graph {
    // Stores a map of brick to bricks that support it
    //
    //   brick -> [bricks...]
    //
    let mut supported_by = HashMap::new();

    // Stores the top z value for each x,y coordinate across all fallen bricks
    //
    //    [x, y] -> (top z, brick)
    //
    let mut top = HashMap::new();

    // Sort bricks by z so that we can process them in order
    bricks.sort_by_cached_key(Brick::min_z);

    // Iterate over every brick...
    for (b, brick) in bricks.iter_mut().enumerate() {
        // For each point in the brick find the top z value in its [x, y]
        // coordinate and compare it to the current z value. If it is higher
        // then update the z value and add the supporting brick to the set.
        let (z, set) = brick
            .iter()
            .filter_map(|p| top.get(&[p.x, p.y]).map(|(z, b)| (z + 1, *b)))
            .fold(
                (0, HashSet::new()),
                |(mut z, mut s), (top_z, top_b): (i64, _)| {
                    match top_z.cmp(&z) {
                        Ordering::Greater => {
                            z = top_z;
                            s.clear();
                            s.insert(top_b);
                        }
                        Ordering::Equal => {
                            s.insert(top_b);
                        }
                        Ordering::Less => {}
                    }
                    (z, s)
                },
            );

        // Fall!
        brick.fall(z);
        for p in brick.iter() {
            top.insert([p.x, p.y], (p.z, b));
        }

        supported_by.insert(b, set);
    }

    supported_by
}

/// Returns all bricks that are the sole support for another brick.
fn dangerous(supported_by: &Graph) -> HashSet<usize> {
    let mut out = HashSet::new();
    for below in supported_by.values() {
        if below.len() != 1 {
            continue;
        }
        let [&b] = below.iter().collect_array();
        out.insert(b);
    }
    out
}

/// Returns the number of bricks that would fall if the given brick was
/// disintegrated.
fn disintegrate(mut supported_by: Vec<usize>, supports: &Graph, start: usize) -> usize {
    let mut q = VecDeque::from([start]);
    let mut fallen = 0;
    while let Some(brick) = q.pop_front() {
        let Some(above) = supports.get(&brick) else {
            // This brick does not support anything, nothing to do
            continue;
        };
        // For every brick above this one decrement the number of bricks
        // that are now supporting it! If the number of bricks supporting it
        // is zero then add it to the queue because it is now falling.
        for &a in above {
            supported_by[a] -= 1;
            if supported_by[a] == 0 {
                q.push_back(a);
                fallen += 1;
            }
        }
    }
    fallen
}

fn part1(bricks: Vec<Brick>) -> usize {
    let n = bricks.len();
    let supported_by = fall(bricks);
    n - dangerous(&supported_by).len()
}

fn part2(bricks: Vec<Brick>) -> usize {
    let n = bricks.len();

    let supported_by = fall(bricks);

    // Invert the "supported by" map by creating a "supports" map. This stores
    // a map of brick to bricks above it.
    let supports = supported_by
        .iter()
        .flat_map(|(&brick, below)| below.iter().map(move |&b| (b, brick)))
        .fold(HashMap::new(), |mut acc, (brick, above)| {
            acc.entry(brick).or_insert_with(HashSet::new).insert(above);
            acc
        });

    let dangerous = dangerous(&supported_by);

    // For simulating the disintegration of bricks we only need to know how many
    // bricks are supporting each brick.
    let supported_by = supported_by
        .iter()
        .fold(vec![0; n], |mut acc, (&b, below)| {
            acc[b] += below.len();
            acc
        });

    // Iterate over each dangerous brick and simulate it disintegrating...
    dangerous
        .into_iter()
        .map(|b| disintegrate(supported_by.clone(), &supports, b))
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    );
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 7);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 465);
    assert_eq!(part2(input), 79042);
}
