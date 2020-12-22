#![feature(min_const_generics)]

pub mod i64 {
    pub type Vector<const N: usize> = vectrs::Vector<i64, N>;

    pub mod xy {
        pub type Vector = vectrs::Vector<i64, 2>;

        pub trait VectorExt {
            fn rotated(self, angle: i64) -> Self;
            fn angle(&self) -> f64;
        }

        impl VectorExt for Vector {
            /// Returns the vector rotated around the origin.
            ///
            /// # Panics
            ///
            /// If the angle is not a multiple of 90 degrees.
            fn rotated(self, angle: i64) -> Self {
                let [x, y] = self.into_array();
                match angle.rem_euclid(360) {
                    0 => Self::new([x, y]),
                    90 => Self::new([-y, x]),
                    180 => Self::new([-x, -y]),
                    270 => Self::new([y, -x]),
                    angle => panic!("called `Vector::rotate()` with oblique angle `{}`", angle),
                }
            }

            /// Returns the angle in radians between the vector and the origin.
            fn angle(&self) -> f64 {
                let [dx, dy] = self.into_array();
                (dy as f64)
                    .atan2(dx as f64)
                    .rem_euclid(std::f64::consts::TAU)
            }
        }
    }
}
