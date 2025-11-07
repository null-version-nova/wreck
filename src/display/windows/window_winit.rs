use crate::display::windows::Window;
use crate::math::vec2::Vector2;

pub struct WindowWinit {
}

impl Window for WindowWinit {
    fn resolution(&self) -> Vector2<u32> {
        todo!()
    }

    fn set_resolution(&mut self, _new: Vector2<u32>) -> bool {
        todo!()
    }

    fn title(&self) -> &str {
        todo!()
    }

    fn set_title(&mut self, _new: &str) -> bool {
        todo!()
    }
}