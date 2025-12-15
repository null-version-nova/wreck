use super::Vector;

use std::{array, cmp::Ordering};

impl<T: Eq, const N: usize> Eq for Vector<T, N> {}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<Vector<U, N>> for Vector<T, N> {
    fn eq(&self, other: &Vector<U, N>) -> bool {
        let mut lhs = self.into_iter();
        let mut rhs = other.into_iter();
        loop {
            match lhs.next() {
                Some(left) => match rhs.next() {
                    Some(right) => {
                        if left != right {
                            return false;
                        }
                    }
                    None => return false,
                },
                None => return true,
            }
        }
    }
}

/// Orders vectors with orderable elements with a partial order.
/// If all orderings are [Ordering::Equal], [Ordering::Equal] is emitted. If some orderings are [Ordering::Less], and none are [None] or [Ordering::Greater], [Ordering::Less] is emitted.
/// If some orderings are [Ordering::Greater], and none are [None] or [Ordering::Less], [Ordering::Greater] is emitted. Else, [None] is emitted.
/// Because of this, a range may be created between two vectors that can enclose a region. This is the only reason why this is defined.
impl<T: PartialOrd<U>, U, const N: usize> PartialOrd<Vector<U, N>> for Vector<T, N> {
    fn partial_cmp(&self, other: &Vector<U, N>) -> Option<Ordering> {
        let mut lhs = self.into_iter();
        let mut rhs = other.into_iter();
        let orderings: [Option<Ordering>; N] =
            array::from_fn(|_| lhs.next().unwrap().partial_cmp(rhs.next().unwrap()));
        let mut ordering = Ordering::Equal;
        for i in orderings {
            match i {
                Some(Ordering::Equal) => continue,
                Some(Ordering::Greater) => {
                    if ordering == Ordering::Less {
                        return None;
                    } else {
                        ordering = Ordering::Greater;
                    }
                }
                Some(Ordering::Less) => {
                    if ordering == Ordering::Greater {
                        return None;
                    } else {
                        ordering = Ordering::Less;
                    }
                }
                None => return None,
            };
        }
        Some(ordering)
    }
}
