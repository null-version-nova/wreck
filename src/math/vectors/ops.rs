pub trait MagnitudeSquared {
    type Output;
    fn mag2(self) -> Self::Output;
}

pub trait Magnitude {
    type Output;
    fn mag(self) -> Self::Output;
}

pub trait Direction {
    type Output;
    fn direction(self) -> Self::Output;
}

pub trait Dot<Other = Self> {
    type Output;
    fn dot(self, other: Other) -> Self::Output;
}

pub trait Cross<Other = Self> {
    type Output;
    fn cross(self, other: Other) -> Self::Output;
}