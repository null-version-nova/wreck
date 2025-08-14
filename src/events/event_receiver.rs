use std::collections::HashSet;

use crate::events::{Event, EventProvider};

pub struct EventReceiver<E: Event + 'static> {
    pub events: Vec<E>,
    providers: HashSet<*mut dyn super::EventProvider<E>>,
}

impl <E: Event + 'static> EventReceiver<E> {
    pub fn register(&self, provider: &mut dyn EventProvider<E>) {
        provider.register(self);
    }

    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            providers: HashSet::new(),
        }
    }
}

impl <E: Event + 'static> super::EventReceiver<E> for EventReceiver<E> {
    fn receive(&mut self, event: E) {
        println!("event {event:?} received!");
        self.events.push(event);
        println!("New event length is {}",self.events.len());
        for i in self.events.iter() {
            println!("{i:?}");
        }
    }
}

impl <E: Event + 'static> Drop for EventReceiver<E> {
    fn drop(&mut self) {
        for i in self.providers.iter() {
            unsafe { (**i).deregister(self); }
        }
    }
}