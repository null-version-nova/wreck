use std::{error::Error, ops::Deref, time::{Duration, Instant}};

use crate::{display::rendermanager::{self, QuitEvent}, events::{ReceiverCell, Event, EventProvider, EventProviderDelegate, EventReceiver}};

pub trait Processing {
    fn process(&mut self,delta: f64) -> Result<(),Box<dyn Error>>;
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
    quit_event_receiver: ReceiverCell<QuitEvent>,
    process_event: EventProviderDelegate<ProcessEvent>,
}

impl Game {
    pub fn new(minimum_delta: f64, process_loop: Box<dyn Fn(f64) -> ()>) -> Game {
        let game = Game {
            minimum_delta: minimum_delta,
            last_delta: minimum_delta,
            running: true,
            process_loop: process_loop,
            quit_event_receiver: EventReceiver::new_cell(),
            process_event: EventProviderDelegate::new()
        };
        rendermanager::INSTANCE.lock().unwrap().register(game.quit_event_receiver.clone());
        game
    }
}

impl Processing for Game {
    fn process(&mut self,delta: f64) -> Result<(),Box<dyn Error>> {
        {
            let _ = self.quit_event_receiver.borrow_mut().execute(&mut |_|{
                self.running = false;
            });
        }
        self.process_event.execute(ProcessEvent { delta });
        self.process_loop.deref()(delta);
        Ok(())
    }
}

impl Process for Game {
    fn run(&mut self) {
        while self.running == true {
            let now = Instant::now();
            let _ = self.process(self.last_delta);
            let after = now.elapsed();
            std::thread::sleep(Duration::from_secs_f64(self.minimum_delta) - after);
            self.last_delta = now.elapsed().as_secs_f64();
        }
    }
}

impl EventProvider<ProcessEvent> for Game {
    fn register(&mut self, receiver: ReceiverCell<ProcessEvent>) {
        self.process_event.register(receiver);
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}