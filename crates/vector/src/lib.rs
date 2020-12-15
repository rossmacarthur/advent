use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl Vector {
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
// Overloaded operators
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Add<$rhs> for $lhs {
            type Output = $output;

            fn add(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
    };
}

impl_add!(Vector, Vector, Vector);
impl_add!(Vector, &Vector, Vector);
impl_add!(&Vector, &Vector, Vector);

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Sub<$rhs> for $lhs {
            type Output = $output;

            fn sub(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }
    };
}

impl_sub!(Vector, Vector, Vector);
impl_sub!(Vector, &Vector, Vector);
impl_sub!(&Vector, &Vector, Vector);

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = $output;

            fn mul(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }
    };
}

impl_mul!(Vector, i64, Vector);
impl_mul!(Vector, &i64, Vector);
impl_mul!(&Vector, i64, Vector);
impl_mul!(&Vector, &i64, Vector);

macro_rules! impl_add_assign {
    ($self:ty, $other:ty) => {
        impl ops::AddAssign<$other> for $self {
            fn add_assign(&mut self, other: $other) {
                self.x += other.x;
                self.y += other.y;
            }
        }
    };
}

impl_add_assign!(Vector, Vector);
impl_add_assign!(Vector, &Vector);

macro_rules! impl_sub_assign {
    ($self:ty, $other:ty) => {
        impl ops::SubAssign<$other> for $self {
            fn sub_assign(&mut self, other: $other) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }
    };
}

impl_sub_assign!(Vector, Vector);
impl_sub_assign!(Vector, &Vector);

////////////////////////////////////////////////////////////////////////////////
// The actual useful stuff
////////////////////////////////////////////////////////////////////////////////

impl Vector {
    /// Rotate the vector around the origin.
    ///
    /// # Panics
    ///
    /// If the angle is not a multiple of 90 degrees.
    pub fn rotated(self, angle: Angle) -> Vector {
        let Self { x, y } = self;
        match angle.normalized() {
            Angle(0) => Self::new(x, y),
            Angle(90) => Self::new(-y, x),
            Angle(180) => Self::new(-x, -y),
            Angle(270) => Self::new(y, -x),
            angle => panic!("called `Vector::rotate()` with oblique angle `{:?}`", angle),
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

////////////////////////////////////////////////////////////////////////////////
// An angle type
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Angle(i64);

impl Angle {
    pub fn new(degrees: i64) -> Self {
        Self(degrees)
    }

    pub fn normalized(self) -> Self {
        Self(self.0.rem_euclid(360))
    }
}
