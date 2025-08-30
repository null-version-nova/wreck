use std::ops::{Add, Mul};

use crate::math::vector_ops::{Dot, Magnitude, MagnitudeSquared};

use super::Vector2;

impl <T: Mul + Copy> MagnitudeSquared for Vector2<T> where T::Output : Add {
    type Output = <T::Output as Add>::Output;

    fn mag2(self) -> Self::Output {
        self.dot(self)
    }
}

impl Magnitude for Vector2<f32> {
    type Output = f32;
    fn mag(self) -> f32 {
        self.mag2().sqrt()
    }
}

impl Magnitude for Vector2<f64> {
    type Output = f64;
    fn mag(self) -> f64 {
        self.mag2().sqrt()
    }
}

impl Magnitude for Vector2<i8> {
    type Output = i8;
    fn mag(self) -> i8 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<i16> {
    type Output = i16;
    fn mag(self) -> i16 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<i32> {
    type Output = i32;
    fn mag(self) -> i32 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<i64> {
    type Output = i64;
    fn mag(self) -> i64 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<i128> {
    type Output = i128;
    fn mag(self) -> i128 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<isize> {
    type Output = isize;
    fn mag(self) -> isize {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<u8> {
    type Output = u8;
    fn mag(self) -> u8 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<u16> {
    type Output = u16;
    fn mag(self) -> u16 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<u32> {
    type Output = u32;
    fn mag(self) -> u32 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<u64> {
    type Output = u64;
    fn mag(self) -> u64 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<u128> {
    type Output = u128;
    fn mag(self) -> u128 {
        self.mag2().isqrt()
    }
}

impl Magnitude for Vector2<usize> {
    type Output = usize;
    fn mag(self) -> usize {
        self.mag2().isqrt()
    }
}
