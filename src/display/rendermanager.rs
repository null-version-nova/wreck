use std::{error::Error, ops::{Index, IndexMut}, sync::Mutex};
use once_cell::sync::Lazy;
use sdl3::event::Event;

use crate::{display::renderers::{window_renderer2::WindowRenderer2, Renderer}, utils::events::{Event as WreckEvent, EventProviderDelegate, ReceiverCell},process::Processing};

#[derive(Clone,Copy,Debug)]
pub struct QuitEvent {}
impl WreckEvent for QuitEvent {}

/// Thread safe rendering singleton!
pub struct RenderManager {
    // Subsystems
    pub sdl: sdl3::Sdl,
    pub video: sdl3::VideoSubsystem,
    pub events: sdl3::EventPump,

    // Events
    quit_event: EventProviderDelegate<QuitEvent>,

    // Data
    windows: Vec<Box<dyn Renderer>>
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
            sdl: process,
            windows: vec![],
            quit_event: EventProviderDelegate::new()
        })
    }

    pub fn get_new_window(&mut self,title: &str, width: u32, height: u32) -> Result<&mut dyn Renderer,Box<dyn Error>> {
        self.windows.push(Box::new(WindowRenderer2::new(&self.video,title,width,height)?));
        Ok(self.windows.last_mut().expect("Just put this here!").as_mut())
    }
}

impl Index<usize> for RenderManager {
    type Output = dyn Renderer;
    fn index(&self, index: usize) -> &Self::Output {
        self.windows[index].as_ref()
    }
}

impl IndexMut<usize> for RenderManager {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.windows[index].as_mut()
    }
}

impl Processing for RenderManager {
    fn process(&mut self,_delta: f64) -> Result<(),Box<dyn Error>> {
        for e in self.events.poll_iter() {
            match e {
                Event::Quit { timestamp: _ } => {
                    self.quit_event.execute(QuitEvent{});
                }
                _ => {}
            }
        }
        for i in &mut self.windows {
            i.draw();
        }
        Ok(())
    }
}

unsafe impl Send for RenderManager {}

impl crate::utils::events::EventProvider<QuitEvent> for RenderManager {
    fn register(&mut self, receiver: ReceiverCell<QuitEvent>) {
        self.quit_event.register(receiver);
    }
}