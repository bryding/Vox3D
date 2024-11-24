use wgpu::util::DeviceExt;

use super::vertex::Vertex;

// Holds rendering data for a single chunk
pub struct ChunkMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}

impl ChunkMesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        );

        Self {
            vertex_buffer,
            num_vertices: vertices.len() as u32,
        }
    }
}
