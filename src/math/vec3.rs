//! # Vector2

// pub mod arithmetic;
// pub mod comparison;
// pub mod conversion;
// pub mod mag;

// #[cfg(test)]
// mod tests;

use std::fmt;

pub trait Magnitude<T> : Copy {
fn mag(self) -> T;
}

#[derive(Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Default> Default for Vector3<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default()
        }
    }
}

impl<T: Clone> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone()
        }
    }
}

impl<T: Copy> Copy for Vector3<T> {}

impl<T: fmt::Display> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}
