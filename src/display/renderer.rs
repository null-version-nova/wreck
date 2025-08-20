pub mod window_renderer2;

/// Owns textures and renders to textures and windows.
pub trait Renderer {
    fn draw(&mut self) -> bool;
}