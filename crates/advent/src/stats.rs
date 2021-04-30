use std::cmp::Ordering;

/// Calculate stats for a collection of data points.
///
/// Note: the provided slice implementation requires that the data is sorted.
pub trait Stats {
    fn min(&self) -> f64;
    fn max(&self) -> f64;
    fn mean(&self) -> f64;
    fn percentile(&self, pct: f64) -> f64;
    fn median(&self) -> f64 {
        self.percentile(50.0)
    }
    fn std_dev(&self) -> f64;
}

pub fn cmp(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap()
}

impl Stats for [f64] {
    fn min(&self) -> f64 {
        self.iter().next().copied().unwrap()
    }

    fn max(&self) -> f64 {
        self.iter().last().copied().unwrap()
    }

    fn mean(&self) -> f64 {
        let sum: f64 = self.iter().sum();
        sum / (self.len() as f64)
    }

    fn percentile(&self, pct: f64) -> f64 {
        let zero: f64 = 0.0;
        let hundred: f64 = 100.0;
        assert!(zero <= pct);
        assert!(pct <= hundred);

        if (pct - hundred).abs() < f64::EPSILON {
            return self[self.len() - 1];
        } else if pct == 0.0 {
            return self[0];
        }

        let len = (self.len() - 1) as f64;
        let rank = (pct / hundred) * len;
        let lrank = rank.floor();
        let d = rank - lrank;
        let n = lrank as usize;
        let lo = self[n];
        let hi = self[n + 1];
        lo + (hi - lo) * d
    }

    fn std_dev(&self) -> f64 {
        let mean = self.mean();
        let sum: f64 = self
            .iter()
            .map(|x| {
                let y = x - mean;
                y * y
            })
            .sum();
        let variance = sum / (self.len() - 1) as f64;
        variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_f64 {
        ($left:expr, $right:expr $(,)?) => {{
            assert!(($left - $right).abs() < f64::EPSILON);
        }};
    }

    #[test]
    fn basic() {
        let nums = [1.0, 2.0, 3.0];
        assert_eq_f64!(nums.min(), 1.0);
        assert_eq_f64!(nums.max(), 3.0);
        assert_eq_f64!(nums.mean(), 2.0);
        assert_eq_f64!(nums.percentile(25.0), 1.5);
        assert_eq_f64!(nums.median(), 2.0);
        assert_eq_f64!(nums.std_dev(), 1.0);
    }
}
