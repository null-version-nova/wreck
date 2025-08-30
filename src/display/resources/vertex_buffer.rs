use sdl3::{gpu::{Buffer, Device, TransferBuffer}, Error};

pub struct VertexBuffer {
    handle: Buffer,
}

impl VertexBuffer {
    pub fn try_new(device: &Device, length: u32) -> Result<Self,Error> {
        Ok(VertexBuffer {
            handle: device.create_buffer().with_usage(sdl3::gpu::BufferUsageFlags::Vertex).with_size(length).build()?,
        })
    }

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