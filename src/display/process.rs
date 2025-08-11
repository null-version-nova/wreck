use std::ops::Deref;

use crate::events::Event;

pub trait Process {
    fn process(&self,delta: f64);
    fn register_event<T: Fn(f64) -> ()>(&mut self,event: T);
    fn run(&self);
}

pub struct Game {
    pub minimum_delta: f64,
    process_event: Event<f64>,
    process_loop: Box<dyn Fn(f64) -> ()>
}

impl Process for Game {
    fn process(&self,delta: f64) {
        self.process_event.call(delta);
        self.process_loop.deref()(delta);
    }

    fn register_event<T: Fn(f64) -> ()>(&mut self,event: T) {
        self.process_event.register(event);
    }

    fn run(&self) {
        todo!()
    }
}