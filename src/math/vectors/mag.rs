use std::ops::{Add, Mul};

use super::ops::{Dot, MagnitudeSquared};

use super::Vector;

impl<T: Mul + Copy> MagnitudeSquared for Vector<T, 2>
where
    T::Output: Add,
{
    type Output = <T::Output as Add>::Output;

    fn mag2(self) -> Self::Output {
        self.dot(self)
    }
}

// impl<const N: usize> Magnitude for Vector<f32, N> {
//     type Output = f32;
//     fn mag(self) -> f32 {
//         self.mag2().sqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<f64, N> {
//     type Output = f64;
//     fn mag(self) -> f64 {
//         self.mag2().sqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<i8, N> {
//     type Output = i8;
//     fn mag(self) -> i8 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<i16, N> {
//     type Output = i16;
//     fn mag(self) -> i16 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<i32, N> {
//     type Output = i32;
//     fn mag(self) -> i32 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<i64, N> {
//     type Output = i64;
//     fn mag(self) -> i64 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<i128, N> {
//     type Output = i128;
//     fn mag(self) -> i128 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<isize, N> {
//     type Output = isize;
//     fn mag(self) -> isize {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<u8, N> {
//     type Output = u8;
//     fn mag(self) -> u8 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<u16, N> {
//     type Output = u16;
//     fn mag(self) -> u16 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<u32, N> {
//     type Output = u32;
//     fn mag(self) -> u32 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<u64, N> {
//     type Output = u64;
//     fn mag(self) -> u64 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<u128, N> {
//     type Output = u128;
//     fn mag(self) -> u128 {
//         self.mag2().isqrt()
//     }
// }

// impl<const N: usize> Magnitude for Vector<usize, N> {
//     type Output = usize;
//     fn mag(self) -> usize {
//         self.mag2().isqrt()
//     }
// }
