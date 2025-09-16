use crate::display::windows::Window;

pub struct WindowGL {

}

impl Window for WindowGL {
    fn resolution(&self) -> crate::math::vec2::Vector2<u32> {
        todo!()
    }

    fn set_resolution(&mut self, new: crate::math::vec2::Vector2<u32>) -> bool {
        todo!()
    }

    fn title(&self) -> &str {
        todo!()
    }

    fn set_title(&mut self, new: &str) -> bool {
        todo!()
    }
}