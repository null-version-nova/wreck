use crate::events::Event;

pub struct EventReceiver<E: Event + 'static> {
    events: Vec<E>,
}

impl <E: Event + 'static> EventReceiver<E> {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn receive(&mut self, event: E) {
        println!("event {event:?} received!");
        self.events.push(event);
        println!("New event length is {}",self.events.len());
        for i in self.events.iter() {
            println!("{i:?}");
        }
    }

    pub fn execute(&mut self, func: &mut dyn FnMut(E) -> ()) {
        for i in self.events.iter_mut() {
            func(*i);
        }
    }
}