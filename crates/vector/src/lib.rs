mod ops;

use std::f64::consts as f64;

/// Represents a two dimensional vector.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl Vector {
    /// Create a new vector.
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl From<(i64, i64)> for Vector {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

////////////////////////////////////////////////////////////////////////////////
// The actual useful stuff
////////////////////////////////////////////////////////////////////////////////

/// Returns the greatest common divisor of two numbers.
fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

impl Vector {
    /// Returns the vector rotated around the origin.
    ///
    /// # Panics
    ///
    /// If the angle is not a multiple of 90 degrees.
    pub fn rotated(self, angle: i64) -> Vector {
        let Self { x, y } = self;
        match angle.rem_euclid(360) {
            0 => Self::new(x, y),
            90 => Self::new(-y, x),
            180 => Self::new(-x, -y),
            270 => Self::new(y, -x),
            angle => panic!("called `Vector::rotate()` with oblique angle `{:?}`", angle),
        }
    }

    /// Returns the distance measured along axes at right angles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vector::Vector;
    /// #
    /// let vector = Vector::new(3, -9);
    /// assert_eq!(vector.manhattan_distance(), 12);
    ///
    /// let dv = Vector::new(2, 5) - Vector::new(1, 3);
    /// assert_eq!(dv.manhattan_distance(), 3);
    /// ```
    pub const fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    /// Reduces the vector to its simplest form.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vector::Vector;
    /// #
    /// let vector = Vector::new(3, -9);
    /// assert_eq!(vector.reduced(), Vector::new(1, -3));
    ///
    /// let vector = Vector::new(0, 5);
    /// assert_eq!(vector.reduced(), Vector::new(0, 1));
    ///
    /// let vector = Vector::new(0, 0);
    /// assert_eq!(vector.reduced(), Vector::new(0, 0));
    /// ```
    pub fn reduced(self) -> Self {
        match self {
            Self { x: 0, y: 0 } => self,
            Self { x, y } => {
                let div = gcd(x, y);
                Self::new(x / div, y / div)
            }
        }
    }

    /// Returns the angle in radians between the vector and the origin.
    pub fn angle(&self) -> f64 {
        let dx = self.x as f64;
        let dy = self.y as f64;
        dy.atan2(dx).rem_euclid(f64::TAU)
    }
}
