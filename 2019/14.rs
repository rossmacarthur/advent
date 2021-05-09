use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

type Pair<'a> = (i64, &'a str);
type Formulae<'a> = HashMap<&'a str, Formula<'a>>;

fn parse_pair(s: &str) -> Pair {
    let (c, chem) = s.split_whitespace().next_tuple().unwrap();
    (c.parse().unwrap(), chem)
}

fn parse_input(input: &str) -> HashMap<&str, Formula> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split(" => ").next_tuple().unwrap();
            let input = lhs.split(", ").map(parse_pair).collect();
            let (result, chem) = parse_pair(rhs);
            (chem, Formula { input, result })
        })
        .collect()
}

fn default_input() -> HashMap<&'static str, Formula<'static>> {
    parse_input(include_str!("input/14.txt"))
}

struct Formula<'a> {
    input: Vec<Pair<'a>>,
    result: i64,
}

fn fuel_to_ore(formulae: &Formulae, fuel: i64) -> i64 {
    let mut need = HashMap::new();
    need.insert("FUEL", fuel);
    loop {
        let mut updates = HashMap::<&str, i64>::new();
        for (chem, amount) in need.iter().filter(|&(_, &a)| a > 0) {
            if let Some(Formula { input, result }) = formulae.get(chem) {
                let n = (amount + result - 1) / result;
                *updates.entry(chem).or_default() -= n * result;
                for (c, chem) in input {
                    *updates.entry(chem).or_default() += n * c;
                }
            }
        }
        if updates.is_empty() {
            break need["ORE"];
        }
        for (chem, amount) in updates {
            *need.entry(chem).or_default() += amount;
        }
    }
}

fn part1(formulae: &Formulae) -> i64 {
    fuel_to_ore(formulae, 1)
}

fn part2(formulae: &Formulae) -> i64 {
    let have = 1_000_000_000_000;
    let mut f0 = have / fuel_to_ore(formulae, 1);
    let mut f1 = 2 * f0;
    loop {
        let fuel = (f0 + f1) / 2;
        if fuel == f0 || fuel == f1 {
            break fuel;
        }
        match fuel_to_ore(formulae, fuel).cmp(&have) {
            Ordering::Equal => break fuel,
            Ordering::Less => f0 = fuel,
            Ordering::Greater => f1 = fuel,
        }
    }
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn example1() {
    let input = parse_input(
        "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
    );
    assert_eq!(part1(&input), 31);
}

#[test]
fn example2() {
    let input = parse_input(
        "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
    );
    assert_eq!(part1(&input), 165);
}

#[test]
fn example3() {
    let input = parse_input(
        "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    );
    assert_eq!(part1(&input), 13312);
    assert_eq!(part2(&input), 82892753);
}

#[test]
fn example4() {
    let input = parse_input(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
    );
    assert_eq!(part1(&input), 180697);
    assert_eq!(part2(&input), 5586022);
}

#[test]
fn example5() {
    let input = parse_input(
        "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
    );
    assert_eq!(part1(&input), 2210736);
    assert_eq!(part2(&input), 460664);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 158482);
    assert_eq!(part2(&input), 7993831);
}
