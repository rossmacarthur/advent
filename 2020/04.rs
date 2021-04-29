use itertools::Itertools;
use regex_macro::regex;

fn parse_input(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| s.split_whitespace().sorted().join(" "))
        .collect()
}

fn default_input() -> Vec<String> {
    parse_input(include_str!("input/04.txt"))
}

fn part1(input: &[String]) -> usize {
    let re = regex!(r"^byr:\S+ (cid:\S+ )?ecl:\S+ eyr:\S+ hcl:\S+ hgt:\S+ iyr:\S+ pid:\S+$");
    input
        .iter()
        .filter(|passport| re.is_match(passport))
        .count()
}

fn part2(input: &[String]) -> usize {
    let re = regex!(
        r"^byr:(19[2-9][0-9]|200[0-2]) (cid:\S+ )?ecl:(amb|blu|brn|gry|grn|hzl|oth) eyr:(202[0-9]|2030) hcl:#[0-9a-f]{6} hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in) iyr:(201[0-9]|2020) pid:\d{9}$"
    );
    input
        .iter()
        .filter(|passport| re.is_match(passport))
        .count()
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
        r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#,
    );
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#,
    );
    assert_eq!(part1(&input), 4);
    assert_eq!(part2(&input), 0);
}

#[test]
fn example3() {
    let input = parse_input(
        r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#,
    );
    assert_eq!(part1(&input), 4);
    assert_eq!(part2(&input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 182);
    assert_eq!(part2(&input), 109);
}
