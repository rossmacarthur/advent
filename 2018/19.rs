mod device;

use device::{compute, parse_program, Program};

fn default_input() -> Program {
    parse_program(include_str!("input/19.txt"))
}

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            sum += i;
            let j = n / i;
            if j != i {
                sum += j
            }
        }
        i += 1
    }
    sum
}

fn part1(prog: Program) -> usize {
    let Program { ip, instrs } = prog;
    let mut regs = [0; 6];
    while regs[ip] < instrs.len() {
        regs = compute(regs, instrs[regs[ip]]);
        regs[ip] += 1;
    }
    regs[0]
}

// Oh no, its too slow. No choice but to deconstruct the program. The important
// part seems to be the following.
//
//   seti 1 . 1    ->    r1 = 1
//   seti 1 . 3    ->    r3 = 1
//   mulr 1 3 5    ->    r5 = r1 * r3          r1 = 1
//   eqrr 5 2 5    ->    r5 = r5 == r2         r3 = 1
//   addr 5 4 4    ->    ip = r5 + ip          while r1 <= r2 {
//   addi 4 1 4    ->    ip = ip + 1               while r3 <= r2 {
//   addr 1 0 0    ->    r0 = r1 + r0                  if r1 * r3 == r2 {
//   addi 3 1 3    ->    r3 = r3 + 1     ->                  r0 += r1
//   gtrr 3 2 5    ->    r5 = r3 > r2                  }
//   addr 4 5 4    ->    ip = ip + r5                  r3 += 1
//   seti 2 . 4    ->    ip = 2                    }
//   addi 1 1 1    ->    r1 += 1                   r1 += 1
//   gtrr 1 2 5    ->    r5 = r1 > r2          }
//   addr 5 4 4    ->    ip = r5 + ip
//   seti 1 . 4    ->    ip = 1
//
// This piece of code of seems to be trying to calculate the sum of factors for
// the current value in r2, storing the result in r0. The rest of the program
// seems to be involved with constructing the value used for the above code and
// involved in part 1.
//
fn part2(prog: Program) -> usize {
    let Program { ip, instrs } = prog;
    let mut regs = [0; 6];
    regs[0] = 1;
    while regs[ip] < instrs.len() {
        if regs[ip] == 2 {
            // Based on the code deconstruction this will be the value that
            // we're calculating the sum of factors for.
            return sum_of_divisors(regs[2]);
        }
        regs = compute(regs, instrs[regs[ip]]);
        regs[ip] += 1;
    }
    unreachable!()
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
    assert_eq!(part1(input.clone()), 1350);
    assert_eq!(part2(input), 15844608);
}
