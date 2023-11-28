use advent::prelude::*;

fn parse_record(input: &str) -> (DateTime<'_>, Event) {
    let re = regex!(r"\[(?P<date>\d{4}-\d{2}-\d{2}) (?P<H>\d{2}):(?P<M>\d{2})\] (?P<descr>.*)");
    let caps = re.captures(input).unwrap();
    let date: &str = caps.name("date").unwrap().as_str();
    let hour: i32 = caps["H"].parse().unwrap();
    let min: i32 = caps["M"].parse().unwrap();
    let datetime = DateTime { date, hour, min };
    let event = match &caps["descr"] {
        "wakes up" => Event::WakesUp,
        "falls asleep" => Event::FallsAsleep,
        d => {
            let id = d
                .strip_prefix("Guard #")
                .unwrap()
                .strip_suffix(" begins shift")
                .unwrap();
            Event::BeginsShift(id.parse().unwrap())
        }
    };
    (datetime, event)
}

fn parse_input(input: &str) -> Vec<(DateTime<'_>, Event)> {
    input.lines().map(parse_record).collect()
}

fn default_input() -> Vec<(DateTime<'static>, Event)> {
    parse_input(include_input!(2018 / 04))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DateTime<'a> {
    date: &'a str,
    hour: i32,
    min: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    BeginsShift(i32),
    FallsAsleep,
    WakesUp,
}

fn solve(mut records: Vec<(DateTime<'_>, Event)>) -> (HashMap<i32, i32>, HashMap<(i32, i32), i32>) {
    let mut totals = HashMap::new();
    let mut mins = HashMap::new();
    let mut guard_id = None;
    let mut asleep = None;
    records.sort();
    for (dt, event) in records {
        match event {
            Event::BeginsShift(id) => guard_id = Some(id),
            Event::FallsAsleep => asleep = Some(dt),
            Event::WakesUp => {
                let guard_id = guard_id.unwrap();
                let asleep = asleep.unwrap();
                for min in asleep.min..dt.min {
                    *mins.entry((guard_id, min)).or_default() += 1;
                    *totals.entry(guard_id).or_default() += 1;
                }
            }
        }
    }
    (totals, mins)
}

fn part1(records: Vec<(DateTime<'_>, Event)>) -> i32 {
    let (totals, mins) = solve(records);
    let (guard_id, _) = totals.iter().max_by_key(|(_, count)| *count).unwrap();
    let ((_, min), _) = mins
        .iter()
        .max_by_key(|((id, _), count)| (id == guard_id, *count))
        .unwrap();
    *guard_id * min
}

fn part2(records: Vec<(DateTime<'_>, Event)>) -> i32 {
    let (_, mins) = solve(records);
    let ((guard_id, min), _) = mins.iter().max_by_key(|(_, count)| *count).unwrap();
    *guard_id * min
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    );
    assert_eq!(part1(input), 240)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 12169);
    assert_eq!(part2(input), 16164);
}
