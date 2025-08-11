//! # Vector2

pub mod arithmetic;
pub mod comparison;
pub mod vec;

#[cfg(test)]
mod tests;

use std::fmt;
use std::ops::{Mul,Add};

pub trait Magnitude<T> : Copy {
fn mag(self) -> T;
}

#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Vector2<T> {
    pub fn into<U: From<T>>(&self) -> Vector2<U> {
        Vector2 {
            x: self.x.into(),
            y: self.x.into()
        }
    }
    pub fn dot<U: Mul<T>>(&self,other: Vector2<U>) -> <<U as Mul<T>>::Output as Add>::Output where <U as Mul<T>>::Output: Add {
        other.x * self.x + other.y * self.y
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone()
        }
    }
}

impl<T: Copy> Copy for Vector2<T> {}

impl<T: fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}
