use std::error::Error;

use crate::{display::windows::Window, math::vec2::Vector2};

pub struct WindowSDL {
    handle: sdl3::video::Window,
}

impl WindowSDL {
    pub fn new(value: &sdl3::VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Self,Box<dyn Error>> {
        Ok(Self {
            handle: value.window(title, width, height).build()?,
        })
    }
}

impl Window for WindowSDL {
    fn resolution(&self) -> Vector2<u32> {
        self.handle.size().into()
    }

    fn set_resolution(&mut self, new: Vector2<u32>) -> bool {
        match self.handle.set_size(new.x, new.y) {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    fn title(&self) -> &str {
        self.handle.title()
    }

    fn set_title(&mut self, new: &str) -> bool {
        match self.handle.set_title(new) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}