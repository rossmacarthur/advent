const INPUT: &str = include_str!("input/day02.txt");

pub fn default_input() -> Vec<usize> {
    INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Computer {
    mem: Vec<usize>,
    ptr: usize,
}

impl Computer {
    fn new(program: Vec<usize>) -> Self {
        Self {
            mem: program,
            ptr: 0,
        }
    }

    fn param(&self, i: usize) -> usize {
        let ptr = self.mem[self.ptr + i];
        self.mem[ptr]
    }

    fn param_mut(&mut self, i: usize) -> &mut usize {
        let ptr = self.mem[self.ptr + i];
        &mut self.mem[ptr]
    }

    fn run(&mut self) {
        loop {
            match self.mem[self.ptr] {
                1 => {
                    *self.param_mut(3) = self.param(1) + self.param(2);
                    self.ptr += 4;
                }
                2 => {
                    *self.param_mut(3) = self.param(1) * self.param(2);
                    self.ptr += 4;
                }
                99 => break,
                opcode => panic!("unknown opcode `{}`", opcode),
            }
        }
    }
}

pub fn part1(input: &[usize]) -> usize {
    let mut computer = Computer::new(input.to_vec());
    computer.mem[1] = 12;
    computer.mem[2] = 2;
    computer.run();
    computer.mem[0]
}

pub fn part2(input: &[usize]) -> usize {
    for noun in 0..input.len() {
        for verb in 0..input.len() {
            let mut computer = Computer::new(input.to_vec());
            computer.mem[1] = noun;
            computer.mem[2] = verb;
            computer.run();
            if computer.mem[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no valid noun and verb found")
}
