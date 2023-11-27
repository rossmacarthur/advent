use advent::prelude::*;

fn default_input() -> [i64; 2] {
    [6, 2]
}

fn part1([mut p1, mut p2]: [i64; 2]) -> i64 {
    let mut s1 = 0;
    let mut s2 = 0;
    let mut die = 1;
    loop {
        p1 = (p1 + 3 * die + 2).rem_euclid(10) + 1;
        s1 += p1;
        die += 3;
        if s1 >= 1000 {
            break s2 * (die - 1);
        }
        mem::swap(&mut p1, &mut p2);
        mem::swap(&mut s1, &mut s2);
    }
}

fn part2([p1, p2]: [i64; 2]) -> usize {
    // Store a count of currently running games which contain the position of
    // the players on the board and their scores. We store two counts: one for
    // games where player 1 just played and games where player 2 just played.
    let mut games = HashMap::from_iter([([p1, p2, 0, 0], [0, 1])]);
    let mut wins1 = 0;
    let mut wins2 = 0;
    while !games.is_empty() {
        games = games
            .into_iter()
            .flat_map(move |([p1, p2, s1, s2], [c1, c2])| {
                // These are the sum of the 3 rolls and the number of ways tha
                // the die can be rolled to equal that sum. For example a 3 can
                // only be rolled once (1+1+1 = 3), but there are 7 ways of
                // rolling 6: (1+2+3), or (2+1+3), (2+2+2) etc.
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                    .into_iter()
                    .map(move |(roll, n)| {
                        let p1 = (p1 + roll - 1).rem_euclid(10) + 1;
                        let s1 = s1 + p1;
                        if s1 < 21 {
                            // No one has won yet: the game should continue and
                            // we swap the players to alternate turns.
                            Either::Left(([p2, p1, s2, s1], [c2 * n, c1 * n]))
                        } else {
                            // The game has been won: return the number of wins
                            // for each player who won in this position.
                            Either::Right([c1 * n, c2 * n])
                        }
                    })
            })
            .fold(HashMap::new(), |mut acc, result| {
                match result {
                    Either::Left((game, [c1, c2])) => {
                        let [t1, t2] = acc.entry(game).or_default();
                        *t1 += c1;
                        *t2 += c2;
                    }
                    Either::Right([w1, w2]) => {
                        wins1 += w1;
                        wins2 += w2;
                    }
                }
                acc
            });
    }
    max(wins1, wins2)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = [4, 8];
    assert_eq!(part1(input), 739785);
    assert_eq!(part2(input), 444356092776315);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 926610);
    assert_eq!(part2(input), 146854918035875);
}
