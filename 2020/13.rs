use advent::prelude::*;

fn parse_input(input: &str) -> (i64, Vec<(i64, i64)>) {
    let [ts, bus_ids] = input.lines().collect_array();
    let bus_ids = bus_ids
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| match x {
            "x" => None,
            x => Some((i as i64, x.parse().unwrap())),
        })
        .collect();
    (ts.parse().unwrap(), bus_ids)
}

fn default_input() -> (i64, Vec<(i64, i64)>) {
    parse_input(include_input!(2020 / 13))
}

fn part1((ts, bus_ids): (i64, Vec<(i64, i64)>)) -> i64 {
    let mut t = ts;
    loop {
        t += 1;
        for (_, id) in &bus_ids {
            if t % id == 0 {
                return id * (t - ts);
            }
        }
    }
}

fn part2((_, mut bus_ids): (i64, Vec<(i64, i64)>)) -> i64 {
    bus_ids.sort_unstable_by_key(|(_, id)| *id);
    let mut t = 0;
    let mut dt = 1;
    for (i, id) in bus_ids {
        while (t + i) % id != 0 {
            t += dt;
        }
        dt *= id;
    }
    t
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("939\n7,13,x,x,59,x,31,19");
    assert_eq!(part1(input.clone()), 295);
    assert_eq!(part2(input), 1068781);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3246);
    assert_eq!(part2(input), 1010182346291467);
}
