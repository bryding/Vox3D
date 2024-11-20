use bytemuck::{Pod, Zeroable};
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub const VERTEX_LAYOUT: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    // This function creates a description of how vertex data is structured for the GPU
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // How many bytes to move forward for each vertex
            // Gets the size of our Vertex struct in bytes
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            // Tells GPU to move forward one vertex at a time (as opposed to per instance for instanced rendering)
            step_mode: wgpu::VertexStepMode::Vertex,

            // Points to our VERTEX_LAYOUT we defined earlier which specified the position and color attributes
            attributes: &Self::VERTEX_LAYOUT,
        }
    }

    pub fn create_cube_vertices(x: f32, y: f32, z: f32) -> Vec<Vertex> {
        vec![
            // Front face
            Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                color: [0.5, 0.5, 0.5],
            },
            // Back face
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.3, 0.3, 0.3],
            },
            // Top face
            Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                color: [0.8, 0.8, 0.8],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                color: [0.8, 0.8, 0.8],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.8, 0.8, 0.8],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.8, 0.8, 0.8],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                color: [0.8, 0.8, 0.8],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                color: [0.8, 0.8, 0.8],
            },
            // Bottom face
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.2, 0.2, 0.2],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                color: [0.2, 0.2, 0.2],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                color: [0.2, 0.2, 0.2],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                color: [0.2, 0.2, 0.2],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                color: [0.2, 0.2, 0.2],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.2, 0.2, 0.2],
            },
            // Right face
            Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                color: [0.6, 0.6, 0.6],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                color: [0.6, 0.6, 0.6],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.6, 0.6, 0.6],
            },
            Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                color: [0.6, 0.6, 0.6],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                color: [0.6, 0.6, 0.6],
            },
            Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                color: [0.6, 0.6, 0.6],
            },
            // Left face
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.4, 0.4, 0.4],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                color: [0.4, 0.4, 0.4],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                color: [0.4, 0.4, 0.4],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                color: [0.4, 0.4, 0.4],
            },
            Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                color: [0.4, 0.4, 0.4],
            },
            Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                color: [0.4, 0.4, 0.4],
            },
        ]
    }
}
