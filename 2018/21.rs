mod device;

use advent::prelude::*;
use device::{compute, parse_program, Program};

fn default_input() -> Program {
    parse_program(include_str!("input/21.txt"))
}

// If we check the instructions we can see that the only place that the first
// register is used is in the following instructions.
//
//   eqrr 1 0 5    ->    r5 = r1 == r0
//   addr 5 2 2    ->    ip = ip + r5
//   seti 5 . 2    ->    ip = 5
//
// Clearly when r0 is equal to r1 the program will halt, otherwise it jumps back
// (ip = 5). Thus, setting r0 to the value of r1 at this point would make the
// program halt.
fn part1(prog: Program) -> usize {
    let Program { ip, instrs } = prog;
    let mut regs = [0; 6];
    while regs[ip] < instrs.len() {
        if regs[ip] == 28 {
            return regs[1];
        }
        regs = compute(regs, instrs[regs[ip]]);
        regs[ip] += 1;
    }
    unreachable!()
}

// For part 2 we can just wait for the last r1 value to occur before the
// sequence repeats. This is quite slow though, so let's try deconstructing the
// program.
//
//   seti 123 . 1        ->  r1 = 123
//   bani 1 456 1        ->  r1 = r1 & 456
//   eqri 1 72 1         ->  r1 = r1 == 72         ->   bitwise verification loop
//   addr 1 2 2          ->  ip = ip + r1
//   seti 0 . 2          ->  ip = 0
//
//   seti 0 . 1          ->  r1 = 0
//   bori 1 65536 4      ->  r4 = r1 | 65536
//   seti 16298264 . 1   ->  r1 = 16298264
//
//   bani 4 255 5        ->  r5 = r4 & 0xff
//   addr 1 5 1          ->  r1 = r1 + r5               r1 = f(r1, r4)
//   bani 1 16777215 1   ->  r1 = r1 & 0xff_ffff        if r4 < 256 {
//   muli 1 65899 1      ->  r1 = r1 * 65899       ->
//   bani 1 16777215 1   ->  r1 = r1 & 0xff_ffff        } else {
//   gtir 256 4 5        ->  r5 = r4 < 256
//   addr 5 2 2          ->  ip = ip + r5               }
//   addi 2 1 2          ->  ip = ip + 1
//   seti 27 . 2         ->  ip = 27
//
//   seti 0 . 5          ->  r5 = 0
//   addi 5 1 3          ->  r3 = r5 + 1
//   muli 3 256 3        ->  r3 = r5 * 256
//   gtrr 3 4 3          ->  r3 = r3 > r4
//   addr 3 2 2          ->  ip = ip + r3          ->   // just division
//   addi 2 1 2          ->  ip = ip + 1                r4 /= 256
//   seti 25 . 2         ->  ip = 25
//   addi 5 1 5          ->  r5 = r5 + 1
//   seti 17 . 2         ->  ip = 17
//
//   setr 5 . 4          ->  r4 = r5
//   seti 7 . 2          ->  ip = 7
//   eqrr 1 0 5          ->  r5 = r1 == r0         ->   halt condition
//   addr 5 2 2          ->  ip = ip + r5
//   seti 5 . 2          ->  ip = 5
//
fn part2(_: Program) -> usize {
    let mut seen = HashSet::new();
    let mut r1 = 0;
    let mut prev_r1 = 0;

    let f = |r1, r4| (((r1 + (r4 & 0xff)) & 0xff_ffff) * 65899) & 0xff_ffff;

    loop {
        let mut r4 = r1 | 65536;
        r1 = f(16298264, r4);
        while r4 >= 256 {
            r4 /= 256;
            r1 = f(r1, r4);
        }
        if !seen.insert(r1) {
            return prev_r1;
        }
        prev_r1 = r1;
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3345459);
    assert_eq!(part2(input), 5857354);
}
