use itertools::Itertools;
use regex_macro::regex;

const INPUT: &str = include_str!("input/day04.txt");

pub fn default_input() -> Vec<String> {
    INPUT
        .split("\n\n")
        .map(|s| s.split_whitespace().sorted().join(" "))
        .collect()
}

pub fn part1(input: &[String]) -> usize {
    let re = regex!(r"^byr:\S+ (cid:\S+ )?ecl:\S+ eyr:\S+ hcl:\S+ hgt:\S+ iyr:\S+ pid:\S+$");
    input
        .iter()
        .filter(|passport| re.is_match(passport))
        .count()
}

pub fn part2(input: &[String]) -> usize {
    let re = regex!(
        r"^byr:(19[2-9][0-9]|200[0-2]) (cid:\S+ )?ecl:(amb|blu|brn|gry|grn|hzl|oth) eyr:(202[0-9]|2030) hcl:#[0-9a-f]{6} hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in) iyr:(201[0-9]|2020) pid:\d{9}$"
    );
    input
        .iter()
        .filter(|passport| re.is_match(passport))
        .count()
}
