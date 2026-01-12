use std::error::Error;

pub trait Emit<T: Emitter, A> {
    fn create(emitter: &T, args: A) -> Self;
}

pub trait TryEmit<T: Emitter, A> {
    type Error: Error;

    fn try_create(emitter: &T, args: A) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait Emitter {
    fn emit_with<A, T: Emit<Self, A>>(&self, args: A) -> T
    where
        Self: Sized,
    {
        T::create(self, args)
    }

    fn try_emit_with<A, T: TryEmit<Self, A>>(&self, args: A) -> Result<T, T::Error>
    where
        Self: Sized,
    {
        T::try_create(self, args)
    }

    fn emit<T: Emit<Self, ()>>(&self) -> T
    where
        Self: Sized,
    {
        self.emit_with(())
    }

    fn try_emit<T: TryEmit<Self, ()>>(&self) -> Result<T, T::Error>
    where
        Self: Sized,
    {
        self.try_emit_with(())
    }
}
