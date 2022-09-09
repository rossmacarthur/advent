use advent::prelude::*;

fn parse_input(input: &str) -> (usize, Vec<(Op, [usize; 3])>) {
    let (ip_config, instrs) = input.split_once('\n').unwrap();
    let ipr = ip_config.trim_start_matches("#ip ").parse().unwrap();
    let instrs = instrs
        .lines()
        .map(|line| {
            let [o, a, b, c] = line.split_whitespace().next_array().unwrap();
            let op = match o {
                "addr" => Op::Addr,
                "addi" => Op::Addi,
                "mulr" => Op::Mulr,
                "muli" => Op::Muli,
                "banr" => Op::Banr,
                "bani" => Op::Bani,
                "borr" => Op::Borr,
                "bori" => Op::Bori,
                "setr" => Op::Setr,
                "seti" => Op::Seti,
                "gtir" => Op::Gtir,
                "gtri" => Op::Gtri,
                "gtrr" => Op::Gtrr,
                "eqir" => Op::Eqir,
                "eqri" => Op::Eqri,
                "eqrr" => Op::Eqrr,
                i => panic!("unexpected instruction `{}`", i),
            };
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            let c = c.parse().unwrap();
            (op, [a, b, c])
        })
        .collect();
    (ipr, instrs)
}

fn default_input() -> (usize, Vec<(Op, [usize; 3])>) {
    parse_input(include_str!("input/19.txt"))
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

fn compute(mut regs: [usize; 6], op: Op, a: usize, b: usize, c: usize) -> [usize; 6] {
    let res = match op {
        Op::Addr => regs[a] + regs[b],
        Op::Addi => regs[a] + b,
        Op::Mulr => regs[a] * regs[b],
        Op::Muli => regs[a] * b,
        Op::Banr => regs[a] & regs[b],
        Op::Bani => regs[a] & b,
        Op::Borr => regs[a] | regs[b],
        Op::Bori => regs[a] | b,
        Op::Setr => regs[a],
        Op::Seti => a,
        Op::Gtir => (a > regs[b]) as usize,
        Op::Gtri => (regs[a] > b) as usize,
        Op::Gtrr => (regs[a] > regs[b]) as usize,
        Op::Eqir => (a == regs[b]) as usize,
        Op::Eqri => (regs[a] == b) as usize,
        Op::Eqrr => (regs[a] == regs[b]) as usize,
    };
    regs[c] = res;
    regs
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

fn part1((ip, instrs): (usize, Vec<(Op, [usize; 3])>)) -> usize {
    let mut regs = [0; 6];
    while regs[ip] < instrs.len() {
        let (op, [a, b, c]) = instrs[regs[ip]];
        regs = compute(regs, op, a, b, c);
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
fn part2((ip, instrs): (usize, Vec<(Op, [usize; 3])>)) -> usize {
    let mut regs = [0; 6];
    regs[0] = 1;
    while regs[ip] < instrs.len() {
        if regs[ip] == 2 {
            // Based on the code deconstruction this will be the value that
            // we're calculating the sum of factors for.
            return sum_of_divisors(regs[2]);
        }
        let (op, [a, b, c]) = instrs[regs[ip]];
        regs = compute(regs, op, a, b, c);
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
