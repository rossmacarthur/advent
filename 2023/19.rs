use advent::prelude::*;

fn parse_braced(s: &str) -> (&str, &str) {
    let i = s.find('{').unwrap();
    let j = s.rfind('}').unwrap();
    (&s[..i], &s[i + 1..j])
}

fn parse_dim(s: &str) -> usize {
    "xmas".find(s).unwrap()
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<Rule<'_>>>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rules) = parse_braced(line);
            let rules = rules
                .split(',')
                .map(|r| match r.split_once(':') {
                    None => Rule::Always(r),
                    Some((cond, workflow)) => {
                        if let Some((field, val)) = cond.split_once('<') {
                            Rule::LessThan(workflow, parse_dim(field), val.parse().unwrap())
                        } else if let Some((field, val)) = cond.split_once('>') {
                            Rule::GreaterThan(workflow, parse_dim(field), val.parse().unwrap())
                        } else {
                            panic!("invalid condition `{}'", cond)
                        }
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| {
            let (_, part) = parse_braced(line);
            part.split(',')
                .map(|s| {
                    let (_, v) = s.split_once('=').unwrap();
                    v.parse().unwrap()
                })
                .collect()
        })
        .collect();

    (workflows, parts)
}

fn default_input() -> (HashMap<&'static str, Vec<Rule<'static>>>, Vec<Part>) {
    parse_input(include_input!(2023 / 19))
}

#[derive(Clone, Copy)]
enum Rule<'a> {
    /// Always send the part to this workflow.
    Always(&'a str),
    /// Send the part to this workflow if the dimension is less than the value.
    LessThan(
        &'a str, // workflow
        usize,   // dimension
        i64,     // value
    ),
    /// Send the part to this workflow if the dimension is greater than the value.
    GreaterThan(
        &'a str, // workflow
        usize,   // dimension
        i64,     // value
    ),
}

type Part = Vector4;

/// Represents a range of parts in each dimension.
#[derive(Debug, Clone, Copy)]
struct Range([(i64, i64); 4]);

impl<'a> Rule<'a> {
    /// Applies the rule to a single part returning the next workflow name or
    /// `None`` if the part should be rejected.
    fn apply_to(self, part: Part) -> Option<&'a str> {
        match self {
            Rule::Always(workflow) => Some(workflow),
            Rule::LessThan(workflow, d, v) => (part[d] < v).some(workflow),
            Rule::GreaterThan(workflow, d, v) => (part[d] > v).some(workflow),
        }
    }

    /// Applies the rule to a range of parts returning the next workflow name
    /// or None for one or more parts. The range is split where appropriate.
    fn apply_to_range(self, parts: Range) -> impl Iterator<Item = (Option<&'a str>, Range)> {
        match self {
            Rule::Always(workflow) => Either::Left([(Some(workflow), parts)]),
            Rule::LessThan(workflow, d, v) => {
                let [p1, p2] = parts.split(d, v);
                Either::Right([(Some(workflow), p1), (None, p2)])
            }
            Rule::GreaterThan(workflow, d, v) => {
                let [p1, p2] = parts.split(d, v + 1);
                Either::Right([(None, p1), (Some(workflow), p2)])
            }
        }
        .into_iter()
        .filter(|(_, parts)| parts.is_not_empty())
    }
}

impl Range {
    /// Returns the total number of parts in the range.
    fn len(self) -> i64 {
        self.0.into_iter().map(|(min, max)| max - min).product()
    }

    /// Returns true if the range is not empty.
    fn is_not_empty(self) -> bool {
        self.0.iter().all(|(min, max)| min < max)
    }

    /// Splits the range in a single dimension at the given value.
    fn split(self, d: usize, v: i64) -> [Self; 2] {
        let Self(mut p1) = self;
        let Self(mut p2) = self;
        p1[d].1 = v;
        p2[d].0 = v;
        [Self(p1), Self(p2)]
    }
}

fn part1((workflows, parts): (HashMap<&str, Vec<Rule<'_>>>, Vec<Part>)) -> i64 {
    parts
        .into_iter()
        .filter(|&part| {
            let (mut workflow, mut rule) = ("in", 0);
            loop {
                (workflow, rule) = match workflows[workflow][rule].apply_to(part) {
                    Some("A") => break true,
                    Some("R") => break false,
                    Some(workflow) => (workflow, 0),
                    None => (workflow, rule + 1),
                }
            }
        })
        .flatten()
        .sum()
}

fn part2((workflows, _): (HashMap<&str, Vec<Rule<'_>>>, Vec<Part>)) -> i64 {
    let mut accepted = 0;
    let start = Range([(1, 4001); 4]);
    let mut q = VecDeque::from([("in", 0, start)]);
    while let Some((workflow, rule, parts)) = q.pop_front() {
        match workflow {
            "A" => accepted += parts.len(),
            "R" => continue,
            _ => {
                let nexts =
                    workflows[workflow][rule]
                        .apply_to_range(parts)
                        .map(|(w, ps)| match w {
                            // send to next workflow
                            Some(w) => (w, 0, ps),
                            // send to the next rule in this workflow
                            None => (workflow, rule + 1, ps),
                        });
                q.extend(nexts);
            }
        }
    }
    accepted
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    );
    assert_eq!(part1(input.clone()), 19114);
    assert_eq!(part2(input), 167409079868000);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 376008);
    assert_eq!(part2(input), 124078207789312);
}
