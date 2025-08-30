use std::error::Error;

use sdl3::gpu::Device;

use crate::display::{renderer::Renderer, window_sdl::WindowSDL};

pub struct WindowRenderer3D {
    handle: Device,
    target: Option<WindowSDL>
}

impl WindowRenderer3D {
    pub fn try_new() -> Result<Self, Box<dyn Error>> {
        Ok(WindowRenderer3D {
            handle: Device::new(sdl3::gpu::ShaderFormat::Invalid, false)?,
            target: None
        })
    }

    pub fn target_ref(&self) -> Option<&WindowSDL> {
        self.target.as_ref()
    }

    pub fn target_mut(&mut self) -> Option<&mut WindowSDL> {
        self.target.as_mut()
    }

    pub fn replace_window(&mut self, window: WindowSDL) -> Option<WindowSDL> {
        self.target.replace(window)
    }

    pub fn take_window(&mut self) -> Option<WindowSDL> {
        self.target.take()
    }
}

impl Renderer for WindowRenderer3D {
    fn draw(&mut self) -> bool {
        true
    }
}