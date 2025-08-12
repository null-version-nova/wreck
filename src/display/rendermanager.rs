use std::{any::Any, error::Error, ops::{Index, IndexMut}, sync::Mutex};
use once_cell::sync::Lazy;
use sdl3::event::Event;

use crate::{display::{process::Processing, window::Window}, events::Event as WreckEvent};

/// Thread safe rendering singleton!
pub struct RenderManager {
    // Subsystems
    _sdl: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    events: sdl3::EventPump,

    // Events
    quit_event: crate::events::printing_event::PrintingEvent<()>,

    // Data
    windows: Vec<Window>
}

pub static INSTANCE: Lazy<Mutex<RenderManager>> = Lazy::new(||{
    Mutex::new(RenderManager::new().unwrap())
});

impl RenderManager {
    fn new() -> Result<RenderManager,Box<dyn Error>> {
        let process = sdl3::init()?;
        Ok(RenderManager {
            video: process.video()?,
            events: process.event_pump()?,
            _sdl: process,
            windows: vec![],
            quit_event: crate::events::printing_event::PrintingEvent::new()
        })
    }

    pub fn get_new_window(&mut self,title: &str, width: u32, height: u32) -> Result<&mut Window,Box<dyn Error>> {
        self.windows.push(Window::new(&self.video,title,width,height)?);
        Ok(self.windows.last_mut().expect("Just put this here!"))
    }

    pub fn register_quit_event<T: FnMut(()) -> () + 'static>(&mut self,event: T) {
        self.quit_event.register(event);
    }
}

impl Index<usize> for RenderManager {
    type Output = Window;
    fn index(&self, index: usize) -> &Self::Output {
        &self.windows[index]
    }
}

impl IndexMut<usize> for RenderManager {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.windows[index]
    }
}

impl Processing for RenderManager {
    fn process(&mut self,_delta: f64) {
        for e in self.events.poll_iter() {
            match e {
                Event::Quit { timestamp: _ } => {
                    println!("Trying to quit!");
                    self.quit_event.call(());
                }
                _ => {}
            }
        }
        for i in &mut self.windows {
            i.draw();
        }
    }
}

unsafe impl Sync for RenderManager {}
unsafe impl Send for RenderManager {}