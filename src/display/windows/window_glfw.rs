use glfw::{GlfwReceiver, WindowEvent};

use crate::display::windows::Window;
use crate::math::vec2::Vector2;

pub struct WindowGlfw {
    handle: glfw::PWindow,
    events: GlfwReceiver<(f64,WindowEvent)>
}

impl WindowGlfw {
    pub fn new(mut env: glfw::Glfw,title: &str, width: u32, height: u32) -> Self {
        let (window, events) = env.create_window(width, height, title, glfw::WindowMode::Windowed).unwrap();
        Self {
            handle: window,
            events: events
        }
    }
}

impl Window for WindowGlfw {
    fn resolution(&self) -> Vector2<u32> {
        let (x,y) = self.handle.get_size();
        Vector2 { x: x.try_into().unwrap(), y: y.try_into().unwrap() }
    }

    fn set_resolution(&mut self, new: Vector2<u32>) -> bool {
        self.handle.set_size(new.x as i32, new.y as i32);
        true
    }

    fn title(&self) -> &str {
        ""
    }

    fn set_title(&mut self, new: &str) -> bool {
        self.handle.set_title(new);
        true
    }
}