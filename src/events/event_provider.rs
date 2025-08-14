use std::collections::HashSet;

use crate::events::Event;

pub struct EventProvider<E> {
    receivers: HashSet<*mut dyn super::EventReceiver<E>>,
}

impl <E: Event> EventProvider<E> {
    pub fn execute(&mut self, event: E) {
        for i in self.receivers.iter() {
            println!("{}",self.receivers.len());
            unsafe { (**i).receive(event); };
        }
    }
    pub fn new() -> Self {
        Self {
            receivers: HashSet::new()
        }
    }
}

impl <E: Event> super::EventProvider<E> for EventProvider<E> {
    fn register(&mut self, receiver: *const (dyn super::EventReceiver<E>)) {
        self.receivers.insert(receiver.cast_mut());
    }
    
    fn deregister(&mut self, receiver: *const dyn super::EventReceiver<E>) {
        println!("{}",self.receivers.remove(&receiver.cast_mut()));
    }
}