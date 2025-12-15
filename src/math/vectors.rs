pub mod arithmetic;
pub mod indirection;

use std::{
    array,
    ops::{Index, IndexMut},
};

#[repr(C)]
#[derive(Debug)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Vector { data }
    }
}

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        self.data.clone().into()
    }
}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;

    type IntoIter = array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
