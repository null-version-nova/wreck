use crate::events::{standard_event::StandardEvent, Event};

pub struct PrintingEvent<Args> where Args : Copy {
    base: StandardEvent<Args>
}

impl <Args> PrintingEvent<Args> where Args : Copy {
    pub fn new() -> Self {
        Self { base: StandardEvent::new() }
    }
}

impl <Args> Event<Args> for PrintingEvent<Args> where Args : Copy + std::fmt::Debug {
    fn register<T: FnMut(Args) -> () + 'static>(&mut self, function: T) {
        println!("Callback registered for event");
        self.base.register(function);
    }

    fn call(&mut self, args: Args) {
        println!("Event called with arguments {args:?}");
        self.base.call(args);
    }
}