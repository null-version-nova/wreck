use std::collections::LinkedList;

use crate::events::Event;

pub struct StandardEvent<Args> where Args : Copy {
    functions: LinkedList<Box<dyn FnMut(Args) -> ()>>
}

impl <Args> StandardEvent<Args> where Args : Copy {
    pub fn new() -> StandardEvent<Args> {
        StandardEvent { functions: LinkedList::new() }
    }
}

impl <Args> Event<Args> for StandardEvent<Args> where Args : Copy {
    fn register<T: FnMut(Args) -> () + 'static>(&mut self, function: T) {
        self.functions.push_back(Box::new(function));
    }

    fn call(&mut self, args: Args) {
        for f in &mut self.functions {
            f.as_mut()(args)
        }
    }
}