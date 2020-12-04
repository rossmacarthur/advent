const INPUT: &str = include_str!("input/day03.txt");

pub fn default_input() -> Input {
    let inner = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Square::Empty,
                    '#' => Square::Tree,
                    _ => panic!("unrecognized input character"),
                })
                .collect()
        })
        .collect();
    Input { inner }
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Tree,
}

#[derive(Debug)]
pub struct Input {
    inner: Vec<Vec<Square>>,
}

impl Input {
    fn lookup(&self, row: usize, col: usize) -> &Square {
        let row = row % self.inner.len();
        let col = col % self.inner[0].len();
        &self.inner[row][col]
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub fn count_trees_for_slope(input: &Input, right: usize, down: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;
    while row < input.len() {
        if let Square::Tree = input.lookup(row, col) {
            trees += 1;
        }
        row += down;
        col += right;
    }
    trees
}

pub fn count_trees_single_slope(input: &Input) -> usize {
    count_trees_for_slope(input, 3, 1)
}

pub fn count_trees_many_slopes(input: &Input) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| count_trees_for_slope(input, right, down))
        .product()
}
