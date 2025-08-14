use std::error::Error;

/// A window spawned from [RenderManager](super::rendermanager::RenderManager)
pub struct Window {
    handle: sdl3::render::Canvas<sdl3::video::Window>,
}

impl Window {
    pub fn new(value: &sdl3::VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Self,Box<dyn Error>> {
        Ok(Window {
            handle: value.window_and_renderer(title, width, height)?,
        })
    }

    pub fn draw(&mut self) {
        self.handle.present();
    }
}