use std::ops::{Index, IndexMut};

pub struct ArrayVector<T, const N: usize = 2> {
    data: [T; N],
}

impl<T, const N: usize> Index<usize> for ArrayVector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrayVector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
