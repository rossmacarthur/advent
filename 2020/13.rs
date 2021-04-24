use itertools::Itertools;

const INPUT: &str = include_str!("input/13.txt");

pub fn default_input() -> (i64, Vec<(i64, i64)>) {
    let (timestamp, bus_ids) = INPUT.lines().next_tuple().unwrap();
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

pub fn part1(input: &(i64, Vec<(i64, i64)>)) -> i64 {
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

pub fn part2(input: &(i64, Vec<(i64, i64)>)) -> i64 {
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
