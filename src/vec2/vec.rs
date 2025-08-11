use super::Vector2;

pub trait Magnitude<T> : Copy {
    fn mag(self) -> T;
}

impl Magnitude<f32> for Vector2<f32> {
    fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Magnitude<f64> for Vector2<f64> {
    fn mag(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Magnitude<i8> for Vector2<i8> {
    fn mag(self) -> i8 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<i16> for Vector2<i16> {
    fn mag(self) -> i16 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<i32> for Vector2<i32> {
    fn mag(self) -> i32 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<i64> for Vector2<i64> {
    fn mag(self) -> i64 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<i128> for Vector2<i128> {
    fn mag(self) -> i128 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<isize> for Vector2<isize> {
    fn mag(self) -> isize {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<u8> for Vector2<u8> {
    fn mag(self) -> u8 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<u16> for Vector2<u16> {
    fn mag(self) -> u16 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<u32> for Vector2<u32> {
    fn mag(self) -> u32 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<u64> for Vector2<u64> {
    fn mag(self) -> u64 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<u128> for Vector2<u128> {
    fn mag(self) -> u128 {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}

impl Magnitude<usize> for Vector2<usize> {
    fn mag(self) -> usize {
        (self.x * self.x + self.y * self.y).isqrt()
    }
}
