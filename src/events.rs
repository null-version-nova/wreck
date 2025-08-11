use std::collections::LinkedList;

pub struct Event<Args> where Args : Copy {
    functions: LinkedList<Box<dyn Fn(Args) -> ()>>
}

impl <Args> Event<Args> where Args : Copy {
    pub fn new() -> Event<Args> {
        Event { functions: LinkedList::new() }
    }

    pub fn register<T: Fn(Args) -> () + 'static>(&mut self, function: T) {
        self.functions.push_back(Box::new(function));
    }

    pub fn call(&self, args: Args) {
        for f in &self.functions {
            f.as_ref()(args)
        }
    }
}