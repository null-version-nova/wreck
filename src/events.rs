use std::any::Any;

pub mod standard_event;
pub mod printing_event;

pub trait AbstractEvent {}

pub trait Event<Args> where Args : Copy {
    fn register<T: FnMut(Args) -> () + 'static>(&mut self, function: T);
    fn call(&mut self, args: Args);
}