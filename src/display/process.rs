use std::{ops::Deref, time::{Duration, Instant}};

use crate::{display::rendermanager::{self, QuitEvent}, events::{Event, EventProvider, EventReceiver}};

pub trait Processing {
    fn process(&mut self,delta: f64);
}

pub trait Process : Processing {
    fn run(&mut self);
}

#[derive(Clone,Copy,Debug)]
pub struct ProcessEvent { pub delta: f64 }
impl Event for ProcessEvent {}

pub struct Game {
    pub minimum_delta: f64,
    last_delta: f64,
    running: bool,
    process_loop: Box<dyn Fn(f64) -> ()>,
    quit_event_receiver: crate::events::event_receiver::EventReceiver<QuitEvent>,
    process_event: crate::events::event_provider::EventProvider<ProcessEvent>,
}

impl Game {
    pub fn new(minimum_delta: f64, process_loop: Box<dyn Fn(f64) -> ()>) -> Game {
        let game = Game {
            minimum_delta: minimum_delta,
            last_delta: minimum_delta,
            running: true,
            process_loop: process_loop,
            quit_event_receiver: crate::events::event_receiver::EventReceiver::new(),
            process_event: crate::events::event_provider::EventProvider::new()
        };
        rendermanager::INSTANCE.lock().unwrap().register(&game);
        game
    }
}

impl Processing for Game {
fn process(&mut self,delta: f64) {
        if self.quit_event_receiver.events.len() >= 1 {
            self.running = false;
        }
        self.process_event.execute(ProcessEvent { delta });
        self.process_loop.deref()(delta);
    }
}

impl Process for Game {
    fn run(&mut self) {
        while self.running == true {
            let now = Instant::now();
            self.process(self.last_delta);
            let after = now.elapsed();
            std::thread::sleep(Duration::from_secs_f64(self.minimum_delta) - after);
            self.last_delta = now.elapsed().as_secs_f64();
        }
    }
}

impl EventReceiver<QuitEvent> for Game {
    fn receive(&mut self, event: QuitEvent) {
        println!("event {event:?} received!");
        self.quit_event_receiver.receive(event);
    }
}

impl EventProvider<ProcessEvent> for Game {
    fn register(&mut self, receiver: *const dyn EventReceiver<ProcessEvent>) {
        self.process_event.register(receiver);
    }

    fn deregister(&mut self, receiver: *const dyn EventReceiver<ProcessEvent>) {
        self.process_event.deregister(receiver);
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}