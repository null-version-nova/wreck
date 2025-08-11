use std::{error::Error, ops::{Index, IndexMut}, sync::Mutex};
use once_cell::sync::Lazy;

use crate::display::window::Window;

/// Thread safe rendering singleton!
pub struct RenderManager {
    sdl: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
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
            sdl: process,
            windows: vec![]
        })
    }

    pub fn get_new_window(&mut self,title: &str, width: u32, height: u32) -> Result<&mut Window,Box<dyn Error>> {
        self.windows.push(Window::new(&self.video,title,width,height)?);
        Ok(self.windows.last_mut().expect("Just put this here!"))
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

unsafe impl Sync for RenderManager {}
unsafe impl Send for RenderManager {}