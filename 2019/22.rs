fn parse_input(input: &str) -> Vec<Method> {
    input
        .lines()
        .map(|line| match line {
            "deal into new stack" => Method::Stack,
            s => {
                let value: i128 = s
                    .rsplit(char::is_whitespace)
                    .map(str::parse)
                    .map(Result::unwrap)
                    .next()
                    .unwrap();
                if s.starts_with("deal with increment") {
                    Method::Deal(value)
                } else if s.starts_with("cut") {
                    Method::Cut(value)
                } else {
                    panic!("unexpected line `{}`", line)
                }
            }
        })
        .collect()
}

fn default_input() -> Vec<Method> {
    parse_input(include_str!("input/22.txt"))
}

/// Represents a shuffle function by storing the constants (a, b) in the
/// following formula.
///
///     f(x) = (ax + b) mod m
///
type Fx = (i128, i128);

/// The identity shuffle function.
const I: Fx = (1, 0);

/// Represents a method of shuffling the deck.
#[derive(Debug, Clone, Copy)]
enum Method {
    Stack,
    Deal(i128),
    Cut(i128),
}

impl Method {
    /// For each shuffle method the next position of a card can be written as a
    /// function of the current position x.
    ///
    ///   "deal into new stack"   => f(x) =  m - x - 1
    ///   "deal with increment v" => f(x) =  vx mod m
    ///   "cut v"                 => f(x) =  (x - v) mod m
    ///
    /// Additionally, each of these transformations can be written in the
    /// following form.
    ///
    ///     f(x) = (ax + b) mod m
    ///
    ///  "deal into new stack"   => f(x) = (-x - 1) mod m
    ///  "deal with increment v" => f(x) = (vx + 0) mod m
    ///  "cut v"                 => f(x) = ( x - v) mod m
    ///
    fn fx(self) -> Fx {
        match self {
            Self::Stack => (-1, -1),
            Self::Deal(v) => (v, 0),
            Self::Cut(v) => (1, -v),
        }
    }
}

/// Composes two shuffle functions together. For example, given the following.
///
///     f(x) = (ax + b) mod m
///     g(x) = (cx + d) mod m
///
/// We can find the result of applying first f then g by substitution.
///
///     g(f(x)) = (acx + bc + d) mod m
///
fn compose(fx: Fx, gx: Fx, m: i128) -> Fx {
    let (a, b) = fx;
    let (c, d) = gx;
    ((a * c).rem_euclid(m), (b * c + d).rem_euclid(m))
}

/// The core logic for reducing all the methods and iterations to a single
/// function of card position.
///
/// This function first finds f by composing all the methods into a single
/// function. Then it finds the result of composing f into itself k times.
fn make_fk(methods: Vec<Method>, mut k: i128, m: i128) -> Fx {
    let mut fx = methods
        .into_iter()
        .map(Method::fx)
        .fold(I, |f, g| compose(f, g, m));
    let mut rx = I;
    while k > 0 {
        if k % 2 == 1 {
            rx = compose(rx, fx, m);
        }
        k /= 2;
        fx = compose(fx, fx, m);
    }
    rx
}

/// Computes the modular multiplicative inverse of a mod m.
///
/// i.e. finds an integer r such that ar mod m = 1 is satisfied.
///
/// We can cheat a little because we know that m will always be prime.
/// See <https://en.wikipedia.org/wiki/Fermat%27s_little_theorem>
fn inv(mut a: i128, m: i128) -> i128 {
    let mut k = m - 2;
    let mut r = 1;
    while k > 0 {
        if k % 2 == 1 {
            r = (r * a).rem_euclid(m);
        }
        a = (a * a).rem_euclid(m);
        k /= 2;
    }
    r
}

fn part1(methods: Vec<Method>) -> i128 {
    const M: i128 = 10007;
    const K: i128 = 1;
    let (a, b) = make_fk(methods, K, M);
    (a * 2019 + b).rem_euclid(M)
}

/// Applying each method is now too computationally expensive so we must first
/// compose all the methods into a single function.
///
/// Given the following
///
///     f(x) = (ax + b) mod m
///     g(x) = (cx + d) mod m
///
/// We can find the result of applying first f then g.
///
///     g(f(x)) = (acx + bc + d) mod m
///
/// Thus we can easily reduce our sequence of shuffle methods into a single
/// function of the cards position.
fn part2(methods: Vec<Method>) -> i128 {
    const M: i128 = 119315717514047;
    const K: i128 = 101741582076661;
    let (a, b) = make_fk(methods, K, M);
    ((2020 - b) * inv(a, M)).rem_euclid(M)
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[cfg(test)]
fn shuffle(methods: Vec<Method>, m: i128) -> Vec<i128> {
    let tree: std::collections::BTreeMap<_, _> = (0..m)
        .map(|i| {
            let (a, b) = make_fk(methods.clone(), 1, m);
            ((a * i + b).rem_euclid(m), i)
        })
        .collect();
    tree.into_values().collect()
}

#[test]
fn example1() {
    let methods = parse_input(
        "\
deal with increment 7
deal into new stack
deal into new stack",
    );
    assert_eq!(shuffle(methods, 10), [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
}

#[test]
fn example2() {
    let methods = parse_input(
        "\
cut 6
deal with increment 7
deal into new stack",
    );
    assert_eq!(shuffle(methods, 10), [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
}

#[test]
fn example3() {
    let methods = parse_input(
        "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
    );
    assert_eq!(shuffle(methods, 10), [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3293);
    assert_eq!(part2(input), 54168121233945);
}
