use itertools::Itertools;

fn parse_input(input: &str) -> (i64, Vec<(i64, i64)>) {
    let (timestamp, bus_ids) = input.lines().next_tuple().unwrap();
    let bus_ids = bus_ids
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| match x {
            "x" => None,
            x => Some((i as i64, x.parse::<i64>().unwrap())),
        })
        .collect();
    (timestamp.parse().unwrap(), bus_ids)
}

fn default_input() -> (i64, Vec<(i64, i64)>) {
    parse_input(include_str!("input/13.txt"))
}

fn part1(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    let (timestamp, bus_ids) = input;
    let mut t = *timestamp;
    'outer: loop {
        t += 1;
        for (_, id) in bus_ids {
            if t % id == 0 {
                break 'outer id * (t - timestamp);
            }
        }
    }
}

fn part2(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    let mut bus_ids = input.clone().1;
    bus_ids.sort_by_key(|(_, id)| *id);
    let mut t = 0;
    let mut dt = 1;
    for (i, id) in &bus_ids {
        while (t + i) % id != 0 {
            t += dt;
        }
        dt *= id;
    }
    t
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("939\n7,13,x,x,59,x,31,19");
    assert_eq!(part1(&input), 295);
    assert_eq!(part2(&input), 1068781);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 3246);
    assert_eq!(part2(&input), 1010182346291467);
}
