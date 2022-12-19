use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            match it.next().unwrap() {
                "noop" => Instr::Noop,
                "addx" => {
                    let n = it.next().unwrap().parse().unwrap();
                    Instr::Addx(n)
                }
                i => panic!("unknown instruction `{i}`"),
            }
        })
        .collect()
}

fn default_input() -> Vec<Instr> {
    parse_input(include_str!("input/10.txt"))
}

#[derive(Clone)]
enum Instr {
    Noop,
    Addx(i64),
}

/// Returns an iterator over the cycle and the X value for that cycle.
fn signal(instrs: Vec<Instr>) -> impl Iterator<Item = (i64, i64)> {
    // An iterator over the change in x value (dx)
    let it = [0]
        .into_iter()
        .chain(instrs.into_iter().flat_map(|instr| match instr {
            Instr::Noop => Either::Left([0].into_iter()),
            Instr::Addx(n) => Either::Right([0, n].into_iter()),
        }));

    // Accumulates the change in x value and enumerates it to get the cycle
    it.scan(1, |x, dx| {
        *x += dx;
        Some(*x)
    })
    .enumerate()
    .map(|(i, x)| (i as i64, x))
}

fn part1(instrs: Vec<Instr>) -> i64 {
    signal(instrs)
        .filter_map(|(i, x)| {
            let cycle = i + 1;
            [20, 60, 100, 140, 180, 220]
                .contains(&cycle)
                .some(cycle * x)
        })
        .sum()
}

fn part2(dxs: Vec<Instr>) -> String {
    let mut s = String::new();
    for (i, x) in signal(dxs) {
        let c = i % 40;
        if c == 0 {
            s.push('\n')
        }
        if c >= x - 1 && c <= x + 1 {
            s.push_str("██")
        } else {
            s.push_str("  ")
        }
    }
    s
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );
    assert_eq!(part1(input.clone()), 13140);
    assert_eq!(
        trim_ends(part2(input)),
        "
████    ████    ████    ████    ████    ████    ████    ████    ████    ████
██████      ██████      ██████      ██████      ██████      ██████      ██████
████████        ████████        ████████        ████████        ████████
██████████          ██████████          ██████████          ██████████
████████████            ████████████            ████████████            ████████
██████████████              ██████████████              ██████████████
"
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 17840);
    assert_eq!(
        trim_ends(part2(input)),
        "
████████    ████    ██          ████    ██    ██  ██        ██████      ████
██        ██    ██  ██        ██    ██  ██    ██  ██        ██    ██  ██    ██
██████    ██    ██  ██        ██        ██    ██  ██        ██    ██  ██
██        ████████  ██        ██  ████  ██    ██  ██        ██████    ██  ████
██        ██    ██  ██        ██    ██  ██    ██  ██        ██        ██    ██
████████  ██    ██  ████████    ██████    ████    ████████  ██          ██████
"
    );
}

#[cfg(test)]
fn trim_ends(s: String) -> String {
    s.lines().map(|line| line.trim_end()).join("\n")
}
