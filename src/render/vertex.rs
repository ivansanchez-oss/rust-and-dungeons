use crate::game::Player;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: Self::SIZE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[derive(Default)]
pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    current: u32,
}

impl QuadBufferBuilder {
    pub fn push_player(&mut self, player: &Player) {
        self.push_quad(
            player.position()[0] - player.size()[0] * 0.5,
            player.position()[1] - player.size()[1] * 0.5,
            player.position()[0] + player.size()[0] * 0.5,
            player.position()[1] + player.size()[1] * 0.5,
        )
    }

    pub fn push_quad(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        self.vertex_data.extend(&[
            Vertex {
                position: [min_x, min_y, 0.0],
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [max_x, min_y, 0.0],
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [max_x, max_y, 0.0],
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [min_x, max_y, 0.0],
                color: [1.0, 1.0, 1.0],
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
