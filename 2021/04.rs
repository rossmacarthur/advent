use advent::prelude::*;

type Board = Matrix<(i64, bool), 5, 5>;

fn parse_input(input: &str) -> (Vec<i64>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws = draws
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(str::parse)
                        .map(Result::unwrap)
                        .map(|n| (n, false))
                })
                .collect()
        })
        .collect();
    (draws, boards)
}

fn default_input() -> (Vec<i64>, Vec<Board>) {
    parse_input(include_input!(2021 / 04))
}

fn update(board: Board, d: i64) -> Board {
    board
        .into_iter()
        .map(|(n, marked)| (n, marked || n == d))
        .collect()
}

fn is_win(board: &Board) -> bool {
    let row_win = board
        .iter_rows()
        .any(|row| row.iter().all(|(_, marked)| *marked));
    let col_win = board
        .iter_columns()
        .any(|col| col.iter().all(|(_, marked)| *marked));
    row_win || col_win
}

fn score(board: &Board, d: i64) -> i64 {
    let sum: i64 = board
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(n, _)| n)
        .sum();
    sum * d
}

fn part1((draws, mut boards): (Vec<i64>, Vec<Board>)) -> i64 {
    for d in draws {
        for board in boards.iter_mut() {
            *board = update(*board, d);
            if is_win(board) {
                return score(board, d);
            }
        }
    }
    unreachable!()
}

fn part2((draws, mut boards): (Vec<i64>, Vec<Board>)) -> i64 {
    for d in draws {
        if let [board] = *boards {
            return score(&update(board, d), d);
        }
        boards = boards
            .into_iter()
            .map(|board| update(board, d))
            .filter(|board| !is_win(board))
            .collect();
    }
    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn example() {
    let input = parse_input(
        "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    );
    assert_eq!(part1(input.clone()), 4512);
    assert_eq!(part2(input), 1924);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 55770);
    assert_eq!(part2(input), 2980);
}
