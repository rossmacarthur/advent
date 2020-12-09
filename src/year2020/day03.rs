const INPUT: &str = include_str!("input/day03.txt");

pub fn default_input() -> Map {
    Map(INPUT
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
        .collect())
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Tree,
}

#[derive(Debug)]
pub struct Map(Vec<Vec<Square>>);

impl Map {
    fn lookup(&self, row: usize, col: usize) -> &Square {
        let row = row % self.0.len();
        let col = col % self.0[0].len();
        &self.0[row][col]
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn count_trees_for_slope(map: &Map, right: usize, down: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;
    while row < map.len() {
        if let Square::Tree = map.lookup(row, col) {
            trees += 1;
        }
        row += down;
        col += right;
    }
    trees
}

pub fn part1(map: &Map) -> usize {
    count_trees_for_slope(map, 3, 1)
}

pub fn part2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| count_trees_for_slope(map, right, down))
        .product()
}
