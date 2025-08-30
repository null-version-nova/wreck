use super::Vector2;

use std::cmp::{
    Ordering,
};

impl<T: Eq> Eq for Vector2<T> {}

impl<T: PartialEq<U>,U> PartialEq<Vector2<U>> for Vector2<T> {
    fn eq(&self, other: &Vector2<U>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Orders vectors with orderable elements with a partial order.
/// If both elements agree on their ordering, that is the order emitted.
/// If both elements disagree, and one of them is equal, the other's ordering is emitted.
/// If both elements disagree, and neither of them are equal, then no ordering is emitted.
impl<T: PartialOrd<U>,U> PartialOrd<Vector2<U>> for Vector2<T> {
    fn partial_cmp(&self, other: &Vector2<U>) -> Option<Ordering> {
        let xcmp = self.x.partial_cmp(&other.x);
        let ycmp = self.y.partial_cmp(&other.y);
        if xcmp? == ycmp? {
            xcmp
        } else {
            if xcmp?.is_eq() {
                ycmp
            } else if ycmp?.is_eq() {
                xcmp
            } else {
                None
            }
        }
    }
}
