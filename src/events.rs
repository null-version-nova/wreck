pub mod event_provider;
pub mod event_receiver;

pub use event_provider::EventProvider as EventProviderDelegate;
pub use event_receiver::EventReceiver as EventReceiverDelegate;

use std::fmt::Debug;

pub trait Event : Copy + Debug {}

pub trait EventProvider<E: Event> {
    fn register(&mut self, receiver: *const dyn EventReceiver<E>);
    fn deregister(&mut self, receiver: *const dyn EventReceiver<E>);
}

/// Registers, receives, and executes events
/// Drop MUST be implemented and call `deregister` on every provider previously registered to. Delegated implementations will handle this automatically, but custom implementations will have to handle this themselves. Failure to do so will lead to arbitray memory mutation, which could pose a security risk. An implementation is not mandated on the trait level to allow for delegates to handle it themselves.
pub trait EventReceiver<E: Event> {
    fn receive(&mut self, event: E);
}