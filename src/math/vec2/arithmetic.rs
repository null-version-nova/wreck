use std::ops::{
    Neg,
    Not,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Rem,
    RemAssign,
};
use crate::math::vector_ops::Dot;

use super::Vector2;

impl <T: Mul<U> + Copy,U> Dot<Vector2<U>> for Vector2<T> where T::Output : Add {
    type Output = <T::Output as Add>::Output;

    fn dot(self,other: Vector2<U>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

// Unary Operations

impl<T: Neg> Neg for Vector2<T> {
    type Output = Vector2<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T: Not> Not for Vector2<T> {
    type Output = Vector2<<T as Not>::Output>;

    fn not(self) -> Self::Output {
        Self::Output {
            x: !self.x,
            y: !self.y
        }
    }
}

// Additive Binary Operations

impl<T: Add<U>,U> Add<Vector2<U>> for Vector2<T> {
    type Output = Vector2<<T as Add<U>>::Output>;

    fn add(self, rhs: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign<U>,U> AddAssign<Vector2<U>> for Vector2<T> {
    fn add_assign(&mut self, rhs: Vector2<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<U>,U> Sub<Vector2<U>> for Vector2<T> {
    type Output = Vector2<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign<U>,U> SubAssign<Vector2<U>> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Vector2<U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// Multiplicative Binary Operations

impl<T: Mul<U>,U: Copy> Mul<U> for Vector2<T> {
    type Output = Vector2<<T as Mul<U>>::Output>;
    
    fn mul(self, rhs: U) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign<U>,U: Copy> MulAssign<U> for Vector2<T> {
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<U>,U: Copy> Div<U> for Vector2<T> {
    type Output = Vector2<<T as Div<U>>::Output>;
    
    fn div(self, rhs: U) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign<U>,U: Copy> DivAssign<U> for Vector2<T> {
    fn div_assign(&mut self, rhs: U) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Rem<U>,U: Copy> Rem<U> for Vector2<T> {
    type Output = Vector2<<T as Rem<U>>::Output>;
    
    fn rem(self, rhs: U) -> Self::Output {
        Self::Output {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T: RemAssign<U>,U: Copy> RemAssign<U> for Vector2<T> {
    fn rem_assign(&mut self, rhs: U) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl<'a,T: Add<&'a U>,U> Add<&'a Vector2<U>> for Vector2<T> {
    type Output = Vector2<<T as Add<&'a U>>::Output>;

    fn add(self, rhs: &'a Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x + &rhs.x,
            y: self.y + &rhs.y,
        }
    }
}

impl<'a,T: AddAssign<&'a U>,U> AddAssign<&'a Vector2<U>> for Vector2<T> {
    fn add_assign(&mut self, rhs: &'a Vector2<U>) {
        self.x += &rhs.x;
        self.y += &rhs.y;
    }
}

impl<'a,T: Sub<&'a U>,U> Sub<&'a Vector2<U>> for Vector2<T> {
    type Output = Vector2<<T as Sub<&'a U>>::Output>;

    fn sub(self, rhs: &'a Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x - &rhs.x,
            y: self.y - &rhs.y,
        }
    }
}

impl<'a,T: SubAssign<&'a U>,U> SubAssign<&'a Vector2<U>> for Vector2<T> {
    fn sub_assign(&mut self, rhs: &'a Vector2<U>) {
        self.x -= &rhs.x;
        self.y -= &rhs.y;
    }
}
