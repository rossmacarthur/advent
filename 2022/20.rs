fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn default_input() -> Vec<i64> {
    parse_input(include_str!("input/20.txt"))
}

fn mix(file: Vec<i64>, times: usize) -> i64 {
    let mut idxs = Vec::from_iter(0..file.len());
    for _ in 0..times {
        for (i, &dj) in file.iter().enumerate() {
            let j = idxs.iter().position(|&n| n == i).unwrap();
            idxs.remove(j);
            let k = (j as i64 + dj).rem_euclid(idxs.len() as i64) as usize;
            idxs.insert(k, i);
        }
    }
    let zero = file.iter().position(|&i| i == 0).unwrap();
    let base = idxs.iter().position(|&i| i == zero).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| file[idxs[(base + i) % idxs.len()]])
        .sum()
}

fn part1(file: Vec<i64>) -> i64 {
    mix(file, 1)
}

fn part2(file: Vec<i64>) -> i64 {
    mix(file.into_iter().map(|x| x * 811589153).collect(), 10)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("1\n2\n-3\n3\n-2\n0\n4");
    assert_eq!(part1(input.clone()), 3);
    assert_eq!(part2(input), 1623178306);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4914);
    assert_eq!(part2(input), 7973051839072);
}
