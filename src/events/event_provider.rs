use crate::events::{Event, ReceiverCell};

pub struct EventProvider<E : Event + 'static> {
    receivers: Vec<ReceiverCell<E>>,
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
    fn register(&mut self, receiver: ReceiverCell<E>) {
        self.receivers.push(receiver);
    }
}