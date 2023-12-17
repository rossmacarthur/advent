use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, i64> {
    parse_map(input, |c| {
        c.to_digit(10)
            .map(|d| d as i64)
            .unwrap_or_else(|| panic!("unexpected character `{c}`"))
    })
}

fn default_input() -> HashMap<Vector2, i64> {
    parse_input(include_input!(2023 / 17))
}

const CARDINALS: [Vector2; 4] = vectors!([0, -1], [0, 1], [-1, 0], [1, 0]);

#[derive(Clone, Copy)]
enum Ordering {
    Equal,
    Reverse,
    Turn,
}

fn cmp_directions(a: Vector2, b: Vector2) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if a == -b {
        Ordering::Reverse
    } else {
        Ordering::Turn
    }
}

fn least_loss(map: HashMap<Vector2, i64>, min_crucible: i64, max_crucible: i64) -> i64 {
    // Perform a Dijkstra shortest path search where each state is a tuple of
    //
    //   (total loss, position, position change since last turn)
    //

    let end = {
        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        vector![max_x, max_y]
    };

    let mut pq: BinaryHeap<_> = CARDINALS
        .into_iter()
        .filter_map(|d| Some((Reverse(map.get(&d).copied()?), d, d)))
        .collect();

    let mut visited = HashSet::new();

    while let Some((Reverse(loss), p, dp)) = pq.pop() {
        if !visited.insert((p, dp)) {
            continue;
        }
        if p == end {
            return loss;
        }
        for &next_d in &CARDINALS {
            let next_p = p + next_d;
            let next_loss = match map.get(&next_p) {
                Some(l) => loss + l,
                None => continue,
            };
            let next_dp = match cmp_directions(next_d, dp.map(i64::signum)) {
                Ordering::Equal if dp.l1_norm() < max_crucible => dp + next_d,
                Ordering::Turn if dp.l1_norm() >= min_crucible => next_d,
                _ => continue,
            };
            pq.push((Reverse(next_loss), next_p, next_dp));
        }
    }

    panic!("no path found")
}

fn part1(map: HashMap<Vector2, i64>) -> i64 {
    least_loss(map, 0, 3)
}

fn part2(map: HashMap<Vector2, i64>) -> i64 {
    least_loss(map, 4, 10)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    );
    assert_eq!(part1(input.clone()), 102);
    assert_eq!(part2(input), 94);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1195);
    assert_eq!(part2(input), 1347);
}
