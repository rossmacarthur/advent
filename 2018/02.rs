use advent::prelude::*;

fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn default_input() -> Vec<&'static str> {
    parse_input(include_input!(2018 / 02))
}

fn part1(input: Vec<&str>) -> i64 {
    let mut freqs: HashMap<char, usize> = HashMap::new();
    let mut twos = 0;
    let mut threes = 0;
    for id in input {
        freqs.clear();
        for c in id.chars() {
            *freqs.entry(c).or_default() += 1;
        }
        twos += freqs.values().any(|&count| count == 2) as i64;
        threes += freqs.values().any(|&count| count == 3) as i64;
    }
    twos * threes
}

fn part2(input: Vec<&str>) -> String {
    for left in &input {
        for right in &input {
            let (m, n) = diff(left, right);
            if m == n {
                return format!("{}{}", &left[..m], &left[n + 1..]);
            }
        }
    }
    unreachable!()
}

// Returns the index of the first and last differing byte(s).
//
// If they are the same the caller can deduce that only one byte is
// different between the strings.
fn diff(left: &str, right: &str) -> (usize, usize) {
    let l = left.as_bytes();
    let r = right.as_bytes();
    let m = (0..l.len()).find(|&i| l[i] != r[i]).unwrap_or(0);
    let n = (0..l.len()).rfind(|&i| l[i] != r[i]).unwrap_or(l.len());
    (m, n)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
    );
    assert_eq!(part1(input), 12)
}

#[test]
fn example2() {
    let input = parse_input(
        "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
    );
    assert_eq!(part2(input), "fgij")
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4712);
    assert_eq!(part2(input), "lufjygedpvfbhftxiwnaorzmq");
}
