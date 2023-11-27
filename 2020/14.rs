use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Instr<'_>> {
    input
        .lines()
        .map(|line| match line.split_once(" = ").unwrap() {
            ("mask", mask) => Instr::Mask(mask.as_bytes()),
            (left, right) => {
                let addr = left
                    .trim_start_matches("mem[")
                    .trim_end_matches(']')
                    .parse()
                    .unwrap();
                Instr::Assign(addr, right.parse().unwrap())
            }
        })
        .collect()
}

fn default_input() -> Vec<Instr<'static>> {
    parse_input(include_str!("input/14.txt"))
}

const DEFAULT_MASK: &[u8] = b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[derive(Debug, Clone)]
enum Instr<'a> {
    Mask(&'a [u8]),
    Assign(i64, i64),
}

/// Converts a mask to a number with floating X values mapped as specified.
fn to_number(mask: &[u8], x: i64) -> i64 {
    mask.iter().fold(0, |acc, b| {
        let d = match b {
            b'0' => 0,
            b'1' => 1,
            b'X' => x,
            b => panic!("unexpected mask value `{b}`"),
        };
        2 * acc + d
    })
}

/// Returns all combinations of an address after applying the given mask.
fn combinations(addr: i64, mask: &[u8]) -> Box<dyn Iterator<Item = i64>> {
    // In the base case we return the unchanged address.
    if mask.is_empty() {
        return Box::new([addr].into_iter());
    }
    // The current mask bit.
    let b = mask[0];
    // The current bit in the address.
    let bit = 1 << (mask.len() - 1);
    // We recurse here to get the combinations of all the rest of the bits.
    let addrs = combinations(addr, &mask[1..]).flat_map(move |addr| {
        // For every combination, we return either one or two
        // addresses dependending on the mask value.
        match b {
            b'0' => Either::Left([addr]),
            b'1' => Either::Left([addr | bit]),
            b'X' => Either::Right([addr, addr ^ bit]),
            b => panic!("unexpected mask value `{b}`"),
        }
        .into_iter()
    });
    Box::new(addrs)
}

fn solve<F, M>(instrs: Vec<Instr<'_>>, f: F) -> i64
where
    F: Fn(&[u8], i64, i64) -> M,
    M: IntoIterator<Item = (i64, i64)>,
{
    let mut memory = HashMap::new();
    let mut mask = DEFAULT_MASK;
    for instr in instrs {
        match instr {
            Instr::Mask(m) => mask = m,
            Instr::Assign(addr, v) => memory.extend(f(mask, addr, v)),
        }
    }
    memory.values().sum()
}

fn part1(instrs: Vec<Instr<'_>>) -> i64 {
    solve(instrs, |mask, addr, v| {
        let or = to_number(mask, 0);
        let and = to_number(mask, 1);
        iter::once((addr, (v | or) & and))
    })
}

fn part2(instrs: Vec<Instr<'_>>) -> i64 {
    solve(instrs, |mask, addr, v| {
        combinations(addr, mask).map(move |addr| (addr, v))
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
    );
    assert_eq!(part1(input), 165);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
    );
    assert_eq!(part2(input), 208);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 11884151942312);
    assert_eq!(part2(input), 2625449018811);
}
