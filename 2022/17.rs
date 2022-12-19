use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vector2> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => vector![1, 0],
            '<' => vector![-1, 0],
            c => panic!("unexpected char `{c}`"),
        })
        .collect()
}

fn default_input() -> Vec<Vector2> {
    parse_input(include_str!("input/17.txt"))
}

const DOWN: Vector2 = vector![0, -1];

// HACK: For shapes with only 4 points I repeated one of the points so that I
//       could use arrays to store the shapes.
const ROCKS: &[[Vector2; 5]] = &[
    vectors!([0, 0], [1, 0], [2, 0], [3, 0], [3, 0]), // Horizontal line
    vectors!([1, 2], [0, 1], [1, 1], [2, 1], [1, 0]), // Plus
    vectors!([2, 2], [2, 1], [2, 0], [1, 0], [0, 0]), // L
    vectors!([0, 3], [0, 2], [0, 1], [0, 0], [0, 0]), // Vertical line
    vectors!([0, 1], [1, 1], [0, 0], [1, 0], [1, 0]), // Square
];

fn simulate(jets: Vec<Vector2>, rocks: i64) -> i64 {
    // Start the tower with just the floor
    let mut tower = HashSet::from_iter((0..7).map(|x| vector![x, 0]));
    // Stores states for tracking cycles
    let mut states = HashMap::new();
    // Tracks the total number of rocks dropped
    let mut r: i64 = 0;
    // Tracks the type of rock
    let mut t = 0;
    // Tracks the jet index
    let mut j = 0;
    // Tracks the height of the tower
    let mut h = 0;
    // For storing any extra height calculated based on a detected cycle. We
    // could add it to `h` but then we would have to shift the entire tower to
    // match.
    let mut h_cycle = 0;

    while r < rocks {
        let mut rock = ROCKS[t].map(|p| p + vector![2, h + 4]);
        t = (t + 1) % ROCKS.len();

        loop {
            // Returns true if a point is not occupied
            let is_open = |p: &Vector2| (0..7).contains(&p.x) && !tower.contains(p);

            // Move with the jet, if it can't move thats fine
            let next = rock.map(|p| p + jets[j]);
            j = (j + 1) % jets.len();
            if next.iter().all(is_open) {
                rock = next;
            }

            // Move with gravity
            let next = rock.map(|p| p + DOWN);
            if next.iter().all(is_open) {
                rock = next;
            } else {
                break;
            }
        }

        // We can no longer move, the rock settles here
        tower.extend(rock);

        // Update the height of the tower
        h = tower.iter().map(|v| v.y).max().unwrap();

        // Check for cycles
        let row: Vector<_, 7> = (0..7).map(|x| tower.contains(&vector![x, h])).collect();
        // Not sure if this key is reliable enough but it works for the example
        // and my input
        let key = (t, j, row);

        if r > 2022 && h_cycle == 0 {
            if let Some((r0, h0)) = states.get(&key) {
                let dr = r - r0;
                let dh = h - h0;
                let count = (rocks - r) / dr;
                r += count * dr;
                h_cycle += count * dh;
            }
        }
        states.insert(key, (r, h));

        r += 1;
    }

    h + h_cycle
}

fn part1(jets: Vec<Vector2>) -> i64 {
    simulate(jets, 2022)
}

fn part2(jets: Vec<Vector2>) -> i64 {
    simulate(jets, 1_000_000_000_000)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    assert_eq!(part1(input.clone()), 3068);
    assert_eq!(part2(input), 1514285714288);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3211);
    assert_eq!(part2(input), 1589142857183);
}
