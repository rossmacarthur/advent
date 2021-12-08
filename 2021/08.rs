use std::ops::BitOr;

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (inputs, outputs) = line.split_once(" | ").unwrap();
            (
                inputs.split_whitespace().map(bitmask).collect(),
                outputs.split_whitespace().map(bitmask).collect(),
            )
        })
        .collect()
}

fn default_input() -> Vec<(Vec<u32>, Vec<u32>)> {
    parse_input(include_str!("input/08.txt"))
}

fn bitmask(s: &str) -> u32 {
    s.bytes().map(|b| 1 << (b - b'a')).fold(0, BitOr::bitor)
}

fn part1(wires: &[(Vec<u32>, Vec<u32>)]) -> usize {
    wires
        .iter()
        .flat_map(|(_, outputs)| outputs)
        .filter(|output| matches!(output.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(wires: Vec<(Vec<u32>, Vec<u32>)>) -> usize {
    wires
        .into_iter()
        .flat_map(|(mut inputs, outputs)| {
            macro_rules! pop {
                ($f:expr) => {{
                    let pos = inputs.iter().position($f).unwrap();
                    inputs.remove(pos)
                }};
            }
            let one = pop!(|i| i.count_ones() == 2);
            let seven = pop!(|i| i.count_ones() == 3);
            let four = pop!(|i| i.count_ones() == 4);
            let eight = pop!(|i| i.count_ones() == 7);
            let nine = pop!(|i| i & four == four);
            let zero = pop!(|i| i.count_ones() == 6 && i & one == one);
            let three = pop!(|i| i.count_ones() == 5 && i & one == one);
            let six = pop!(|i| i.count_ones() == 6);
            let five = pop!(|&i| six & i == i);
            let two = inputs.remove(0);
            let nums = [zero, one, two, three, four, five, six, seven, eight, nine];
            outputs.into_iter().rev().enumerate().map(move |(i, d)| {
                let pos = nums.iter().position(|n| *n == d).unwrap();
                pos * 10_usize.pow(i as u32)
            })
        })
        .sum()
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(input.clone()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    );
    assert_eq!(part1(&input), 26);
    assert_eq!(part2(input), 61229);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 534);
    assert_eq!(part2(input), 1070188);
}
