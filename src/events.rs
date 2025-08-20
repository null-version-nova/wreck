pub mod event_provider;
pub mod event_receiver;

pub use event_provider::EventProvider as EventProviderDelegate;
pub use event_receiver::EventReceiver as EventReceiver;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Event : Copy + Debug {}

pub trait EventProvider<E: Event> {
    fn register(&mut self, receiver: ReceiverCell<E>);
}

pub type ReceiverCell<T> = Rc<RefCell<EventReceiver<T>>>;