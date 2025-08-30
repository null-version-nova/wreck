//! # Vector2

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod mag;

#[cfg(test)]
mod tests;

use std::fmt;

pub trait Magnitude<T> : Copy {
fn mag(self) -> T;
}

#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
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
