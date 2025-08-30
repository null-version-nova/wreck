use std::error::Error;

use crate::{display::{renderer::Renderer, window::Window}, math::vec2::Vector2};

pub struct WindowRenderer2 {
    handle: sdl3::render::Canvas<sdl3::video::Window>,
}

impl WindowRenderer2 {
    pub fn new(value: &sdl3::VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Self,Box<dyn Error>> {
        Ok(Self {
            handle: value.window_and_renderer(title, width, height)?,
        })
    }
}

impl Renderer for WindowRenderer2 {
    fn draw(&mut self) -> bool {
        self.handle.present()
    }
}

impl Window for WindowRenderer2 {
    fn resolution(&self) -> Vector2<u32> {
        self.handle.window().size().into()
    }

    fn set_resolution(&mut self, new: Vector2<u32>) -> bool {
        match self.handle.window_mut().set_size(new.x, new.y) {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    fn title(&self) -> &str {
        self.handle.window().title()
    }

    fn set_title(&mut self, new: &str) -> bool {
        match self.handle.window_mut().set_title(new) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}