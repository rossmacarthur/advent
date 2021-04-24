/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

/// Returns the least common multiple of two numbers.
pub fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}
