use std::ptr::null_mut;

use gl::GenBuffers;

pub struct BufferObject {
    buffer: u32,
}

impl BufferObject {
    pub fn new() -> Self {
        let i: *mut u32 = null_mut();
        unsafe {
            GenBuffers(1, i);
            BufferObject { buffer: *i }
        }
    }
}
