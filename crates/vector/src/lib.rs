pub mod i64 {
    pub type Vector<const N: usize> = vectrix::Vector<i64, N>;

    pub mod xy {
        pub use vectrix::vector;

        pub type Vector = super::Vector<2>;

        /// Returns the greatest common divisor of two numbers.
        fn gcd(mut x: i64, mut y: i64) -> i64 {
            while x != 0 {
                let tmp = x;
                x = y % tmp;
                y = tmp;
            }
            y.abs()
        }

        pub trait VectorExt {
            fn reduced(self) -> Self;
            fn rotated(self, angle: i64) -> Self;
            fn angle(&self) -> f64;
        }

        impl VectorExt for Vector {
            /// Returns the reduced row echelon form of the vector.
            ///
            /// This is the same as dividing each element by the greatest common
            /// divisor of all the elements.
            fn reduced(self) -> Self {
                let x = self.x;
                let y = self.y;
                let div = gcd(x, y);
                vector![x / div, y / div]
            }

            /// Returns the vector rotated around the origin.
            ///
            /// # Panics
            ///
            /// If the angle is not a multiple of 90 degrees.
            fn rotated(self, angle: i64) -> Self {
                let x = self.x;
                let y = self.y;
                match angle.rem_euclid(360) {
                    0 => vector![x, y],
                    90 => vector![-y, x],
                    180 => vector![-x, -y],
                    270 => vector![y, -x],
                    angle => panic!("called `Vector::rotate()` with oblique angle `{}`", angle),
                }
            }

            /// Returns the angle in radians between the vector and the origin.
            fn angle(&self) -> f64 {
                (self.y as f64)
                    .atan2(self.x as f64)
                    .rem_euclid(std::f64::consts::TAU)
            }
        }
    }
}
