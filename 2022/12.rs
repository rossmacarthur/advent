use advent::prelude::*;

fn parse_input(input: &str) -> (HashMap<Vector2, u8>, Vector2, Vector2) {
    let mut map: HashMap<_, _> = parse_map(input, |c| match c {
        'a'..='z' | 'S' | 'E' => c as u8,
        c => panic!("unexpected character `{c}`"),
    });
    let mut update = |b, e| {
        let p = map.iter().find_map(|(&p, &e)| (e == b).some(p)).unwrap();
        map.insert(p, e);
        p
    };
    let start = update(b'S', b'a');
    let end = update(b'E', b'z');
    (map, start, end)
}

fn default_input() -> (HashMap<Vector2, u8>, Vector2, Vector2) {
    parse_input(include_input!(2022 / 12))
}

fn shortest<I>(map: &HashMap<Vector2, u8>, start: I, end: Vector2) -> usize
where
    I: IntoIterator<Item = Vector2>,
{
    let mut q: VecDeque<_> = start.into_iter().map(|p| (p, 0)).collect();
    let mut visited = HashSet::new();
    while let Some((p, steps)) = q.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        if p == end {
            return steps;
        }
        let curr = map[&p];
        for d in vectors!([0, -1], [-1, 0], [0, 1], [1, 0]) {
            let next = p + d;
            if let Some(&elev) = map.get(&next) {
                if elev.saturating_sub(curr) <= 1 {
                    q.push_back((next, steps + 1));
                }
            }
        }
    }
    panic!("no path found")
}

fn part1((map, start, end): (HashMap<Vector2, u8>, Vector2, Vector2)) -> usize {
    shortest(&map, [start], end)
}

fn part2((map, _, end): (HashMap<Vector2, u8>, Vector2, Vector2)) -> usize {
    let start = map.iter().filter_map(|(&p, &e)| (e == b'a').some(p));
    shortest(&map, start, end)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    );
    assert_eq!(part1(input.clone()), 31);
    assert_eq!(part2(input), 29);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 420);
    assert_eq!(part2(input), 414);
}
