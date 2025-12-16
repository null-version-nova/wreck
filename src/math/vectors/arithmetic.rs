use crate::math::vectors::ops::Dot;

use super::Vector;
use std::{
    array,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
    },
};

impl<T: Mul<U> + Copy, U: Copy> Dot<Vector<U,2>> for Vector<T,2>
where
    T::Output: Add,
{
    type Output = <T::Output as Add>::Output;

    fn dot(self, other: Vector<U,2>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

// Unary Operations

impl<T: Neg, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<<T as Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        self.data.map(|i| -i).into()
    }
}

impl<T: Not, const N: usize> Not for Vector<T, N> {
    type Output = Vector<<T as Not>::Output, N>;

    fn not(self) -> Self::Output {
        self.data.map(|i| !i).into()
    }
}

impl<T: Add<U>, U, const N: usize> Add<Vector<U, N>> for Vector<T, N> {
    type Output = Vector<<T as Add<U>>::Output, N>;

    fn add(self, rhs: Vector<U, N>) -> Self::Output {
        let mut lhs_iter = self.into_iter();
        let mut rhs_iter = rhs.into_iter();
        array::from_fn(|_| lhs_iter.next().unwrap() + rhs_iter.next().unwrap()).into()
    }
}

// Binary Operations

impl<T: AddAssign<U>, U, const N: usize> AddAssign<Vector<U, N>> for Vector<T, N> {
    fn add_assign(&mut self, rhs: Vector<U, N>) {
        let mut rhs = rhs.into_iter();
        for idx in 0..N {
            self[idx] += rhs.next().unwrap();
        }
    }
}

impl<T: Sub<U>, U, const N: usize> Sub<Vector<U, N>> for Vector<T, N> {
    type Output = Vector<<T as Sub<U>>::Output, N>;

    fn sub(self, rhs: Vector<U, N>) -> Self::Output {
        let mut lhs_iter = self.into_iter();
        let mut rhs_iter = rhs.into_iter();
        array::from_fn(|_| lhs_iter.next().unwrap() - rhs_iter.next().unwrap()).into()
    }
}

impl<T: SubAssign<U>, U, const N: usize> SubAssign<Vector<U, N>> for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Vector<U, N>) {
        let mut rhs = rhs.into_iter();
        for idx in 0..N {
            self[idx] -= rhs.next().unwrap();
        }
    }
}

impl<T: Mul<U>, U: Copy, const N: usize> Mul<U> for Vector<T, N> {
    type Output = Vector<<T as Mul<U>>::Output, N>;

    fn mul(self, rhs: U) -> Self::Output {
        self.data.map(|i| i * rhs).into()
    }
}

impl<T: MulAssign<U>, U: Copy, const N: usize> MulAssign<U> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: U) {
        for i in &mut self.data {
            *i *= rhs;
        }
    }
}

impl<T: Div<U>, U: Copy, const N: usize> Div<U> for Vector<T, N> {
    type Output = Vector<<T as Div<U>>::Output, N>;

    fn div(self, rhs: U) -> Self::Output {
        self.data.map(|i| i / rhs).into()
    }
}

impl<T: DivAssign<U>, U: Copy, const N: usize> DivAssign<U> for Vector<T, N> {
    fn div_assign(&mut self, rhs: U) {
        for i in &mut self.data {
            *i /= rhs;
        }
    }
}

impl<T: Rem<U>, U: Copy, const N: usize> Rem<U> for Vector<T, N> {
    type Output = Vector<<T as Rem<U>>::Output, N>;

    fn rem(self, rhs: U) -> Self::Output {
        self.data.map(|i| i % rhs).into()
    }
}

impl<T: RemAssign<U>, U: Copy, const N: usize> RemAssign<U> for Vector<T, N> {
    fn rem_assign(&mut self, rhs: U) {
        for i in &mut self.data {
            *i %= rhs;
        }
    }
}
