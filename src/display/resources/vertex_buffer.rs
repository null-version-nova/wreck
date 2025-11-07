use sdl3::{
    Error,
    gpu::{Buffer, TransferBuffer},
};


use crate::display::renderers::window_renderer_3d::WindowRenderer3D;

pub struct VertexBuffer<'s> {
    owner: &'s WindowRenderer3D,
    handle: Buffer,
}

impl<'s> VertexBuffer<'s> {
    pub fn map(&mut self, buffer: &TransferBuffer) -> bool {
        if buffer.len() < self.len() {
            return false;
        }
        true
    }

    pub fn len(&self) -> u32 {
        self.handle.len()
    }
}

impl WindowRenderer3D {
    pub fn create_vertex_buffer(&'_ mut self, length: u32) -> Result<VertexBuffer<'_>, Error> {
        Ok(VertexBuffer {
            owner: self,
            handle: self
                .device_ref()
                .create_buffer()
                .with_usage(sdl3::gpu::BufferUsageFlags::Vertex)
                .with_size(length)
                .build()?,
        })
    }
}
