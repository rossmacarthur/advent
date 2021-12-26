use regex_macro::regex;

fn parse_program(input: &str) -> Vec<[i64; 3]> {
    // The input can be chunked into instructions of the following form, with
    // the only differing values captured by the regex. We'll refer to these
    // values as `a`, `b`, and `c`.
    let re = regex!(
        r"inp w
mul x 0
add x z
mod x 26
div z (-?\d+)
add x (-?\d+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?\d+)
mul y x
add z y"
    );

    let instrs: Vec<_> = re
        .captures_iter(input)
        .map(|caps| {
            let a = caps[1].parse().unwrap();
            let b = caps[2].parse().unwrap();
            let c = caps[3].parse().unwrap();
            [a, b, c]
        })
        .collect();
    assert_eq!(instrs.len(), 14);
    instrs
}

fn default_input() -> Vec<[i64; 3]> {
    parse_program(include_str!("input/24.txt"))
}

fn solve(instrs: Vec<[i64; 3]>) -> (i64, i64) {
    // Each of the 14 sections of instructions can be reduced using the
    // following observations.
    //
    // - `w` is always the input
    // - `x` and `y` always start at zero
    // - `z `is the only value carried through to the next computation
    //
    // So we end up with the following.
    //
    //     x = (z % 26 + b) != w
    //     z = (z / a) * (25 * x + 1) + (w + c) * x
    //
    // Additionally, the provided input has the following properties.
    //
    // - `a` is *always* 1 or 26
    // - there are seven cases of `a` equal to 1 and seven equal to 26.
    // - when `a` is 1, `b` is between 10 and 14
    // - when `a` is 26, `b` is negative
    // - `c` is always between 0 and 14
    //
    // Based on this there are only two ways for `z` to change.
    //
    // 1. When `a` is 1 then `z` will grow by a factor of 26 plus some value
    //    less than 26.
    //
    //      z = 26 * z + (w + c)
    //
    // 2. When `a` is 26, if `x` can be made 0 then `z` will be divided by 26.
    //
    //     z = z / 26
    //
    // Thus, if we want `z` to end up as zero then we need to make sure that
    // when `a` is 26 we have to make `x` zero. This will only happen if we can
    // satisfy the following equation.
    //
    //     z % 26 + b == w
    //
    // We know that the result of `z % 26` will be the "previous" `w + c`
    // (because `w + c` is always less than 26). By previous here we mean the
    // last time `a` was 1. So the constraint equation becomes.
    //
    //     prev_w + prev_c + b == w
    //

    let mut stack = Vec::new();
    let mut min = [0; 14];
    let mut max = [0; 14];
    for (j, &[a, b, c]) in instrs.iter().enumerate() {
        match a {
            // We store the position of the digit and this `c` value so that we
            // can use it to satisfy the constraint later when we encounter the
            // next time `a` is 26.
            1 => stack.push((j, c)),

            // Now we try and satisfy the constraint by popping the previous
            // `c` value and setting the previous `w` and this `w` such that
            // they satisfy the constraint.
            26 => {
                let (i, c) = stack.pop().unwrap();
                let d = b + c;
                let (i, j, d) = if d < 0 { (j, i, -d) } else { (i, j, d) };
                max[i] = 9 - d;
                max[j] = 9;
                min[i] = 1;
                min[j] = 1 + d;
            }
            _ => unreachable!(),
        }
    }
    verify(&instrs, min);
    verify(&instrs, max);
    let min = min.into_iter().fold(0, |acc, d| acc * 10 + d);
    let max = max.into_iter().fold(0, |acc, d| acc * 10 + d);
    (min, max)
}

fn verify(instrs: &[[i64; 3]], digits: [i64; 14]) {
    let mut z = 0;
    for ([a, b, c], w) in instrs.iter().zip(digits) {
        let x = ((z % 26 + b) != w) as i64;
        z = (z / a) * (25 * x + 1) + (w + c) * x;
    }
    assert_eq!(z, 0, "digits `{:?}` are incorrect", digits)
}

fn part1(instrs: Vec<[i64; 3]>) -> i64 {
    let (_, max) = solve(instrs);
    max
}

fn part2(instrs: Vec<[i64; 3]>) -> i64 {
    let (min, _) = solve(instrs);
    min
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 74929995999389);
    assert_eq!(part2(input), 11118151637112);
}
