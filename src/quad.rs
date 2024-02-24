use crate::Vertex;
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    current: u32,
}

impl QuadBufferBuilder {
    pub fn push_quad(mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        self.vertex_data.extend(&[
            Vertex {
                position: [min_x, min_y, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [max_x, min_y, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [max_x, max_y, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [min_x, max_y, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ]);

        self.index_data.extend(&[
            self.current * 4 + 0,
            self.current * 4 + 1,
            self.current * 4 + 2,
            self.current * 4 + 0,
            self.current * 4 + 2,
            self.current * 4 + 3,
        ]);

        self.current += 1;
        self
    }

    pub fn build(self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Temporal Buffer"),
            contents: bytemuck::cast_slice(&self.vertex_data),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::empty(),
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Temporal Buffer"),
            contents: bytemuck::cast_slice(&self.index_data),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer, self.index_data.len() as u32)
    }
}
