use std::rc::Rc;
use std::cell::RefCell;

use super::{Event, ReceiverCell};

pub struct EventReceiver<E: Event + 'static> {
    events: Vec<E>
}

impl <E: Event + 'static> EventReceiver<E> {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    pub fn new_cell() -> ReceiverCell<E> {
        Rc::new(RefCell::new(Self { events: Vec::new() }))
    }

    pub fn receive(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn execute(&mut self, func: &mut dyn FnMut(E) -> ()) {
        for i in self.events.iter_mut() {
            func(*i);
        }
    }
}