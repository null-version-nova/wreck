use crate::vec2::Vector2;

pub trait Window {
    fn resolution(&self) -> Vector2<u32>;

    fn set_resolution(&mut self, new: Vector2<u32>) -> bool;

    fn title(&self) -> &str;

    fn set_title(&mut self, new: &str) -> bool;
}