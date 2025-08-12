use std::{ops::Deref, time::{Duration, Instant}};

use crate::{display::rendermanager, events::{standard_event::StandardEvent, Event}};

pub trait Processing {
    fn process(&mut self,delta: f64);
}

pub trait Process : Processing {
    fn register_process_event<T: FnMut(f64) -> () + 'static>(&mut self,event: T);
    fn run(&mut self);
}

pub struct Game {
    pub minimum_delta: f64,
    last_delta: f64,
    running: bool,
    process_event: StandardEvent<f64>,
    process_loop: Box<dyn Fn(f64) -> ()>
}

impl Game {
    pub fn new(minimum_delta: f64, process_loop: Box<dyn Fn(f64) -> ()>) -> Game {
        let mut game = Game {
            minimum_delta: minimum_delta,
            last_delta: minimum_delta,
            running: true,
            process_event: StandardEvent::new(),
            process_loop: process_loop
        };
        let ptr: *mut Game = &mut game;
        rendermanager::INSTANCE.lock().unwrap().register_quit_event(move |_|{
            unsafe { ptr.read().running = false; }
        });
        game
    }
}

impl Processing for Game {
fn process(&mut self,delta: f64) {
        self.process_event.call(delta);
        self.process_loop.deref()(delta);
    }
}

impl Process for Game {
    fn register_process_event<T: FnMut(f64) -> () + 'static>(&mut self,event: T) {
        self.process_event.register(event);
    }

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