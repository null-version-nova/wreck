use super::Vector2;

impl<T> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    fn from(value: Vector2<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T> {
    fn from(value: [T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T> From<Vector2<T>> for [T; 2] {
    fn from(value: Vector2<T>) -> Self {
        [value.x, value.y]
    }
}

/// Generic conversion methods. Due to trait collisions this cannot be a blanket impl for [From] and [TryFrom] but this should be about as good.
impl<T> Vector2<T> {
    pub fn into<U>(self) -> Vector2<U>
    where
        T: Into<U>,
    {
        Vector2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
    pub fn from<U: Into<T>>(other: Vector2<U>) -> Vector2<T> {
        Self {
            x: other.x.into(),
            y: other.y.into(),
        }
    }

    pub fn try_into<U>(self) -> Result<Vector2<U>, T::Error>
    where
        T: TryInto<U>,
    {
        Ok(Vector2 {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }
    pub fn try_from<U: TryInto<T>>(other: Vector2<U>) -> Result<Vector2<T>, U::Error> {
        Ok(Self {
            x: other.x.try_into()?,
            y: other.y.try_into()?,
        })
    }
}
