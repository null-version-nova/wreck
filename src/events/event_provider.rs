use std::{cell::RefCell, rc::Rc};

use crate::events::{Event, EventReceiver};

pub struct EventProvider<E : Event + 'static> {
    receivers: Vec<Rc<RefCell<EventReceiver<E>>>>,
}

impl <E: Event> EventProvider<E> {
    pub fn execute(&mut self, event: E) {
        for i in self.receivers.iter_mut() {
            i.borrow_mut().receive(event);
        }
    }
    pub fn new() -> Self {
        Self {
            receivers: Vec::new()
        }
    }
}

impl <E: Event> super::EventProvider<E> for EventProvider<E> {
    fn register(&mut self, receiver: Rc<RefCell<super::EventReceiver<E>>>) {
        self.receivers.push(receiver);
    }
}