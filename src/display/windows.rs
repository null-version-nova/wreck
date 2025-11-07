pub mod window_gl;
pub mod window_glfw;
pub mod window_sdl;
pub mod window_winit;

use crate::math::vec2::Vector2;

pub trait Window {
    fn resolution(&self) -> Vector2<u32>;

    fn set_resolution(&mut self, new: Vector2<u32>) -> bool;

    fn title(&self) -> &str;

    fn set_title(&mut self, new: &str) -> bool;
}