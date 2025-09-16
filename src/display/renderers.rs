pub mod window_renderer2;
pub mod window_renderer_3d;

/// Owns resources and renders to textures and windows.
pub trait Renderer {
    fn draw(&mut self) -> bool;
}