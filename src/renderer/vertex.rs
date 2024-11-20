use wgpu;
use bytemuck::{ Pod, Zeroable };

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub const VERTEX_LAYOUT: [
        wgpu::VertexAttribute;
        3
    ] = wgpu::vertex_attr_array![
            0 => Float32x3,  // position
            1 => Float32x3,  // color
            2 => Float32x3,  // normal
        ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_LAYOUT,
        }
    }

    pub fn create_cube_vertices(x: f32, y: f32, z: f32, color: [f32; 3]) -> Vec<Vertex> {
        vec![
            // Front face
            Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },
            Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },
            Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },
            Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal: [0.0, 0.0, 1.0] },

            // Back face
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },
            Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },
            Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },
            Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },
            Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [0.0, 0.0, -1.0] },

            // Top face
            Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 1.0, 0.0] },
            Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 1.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 1.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [0.0, 1.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 1.0, 0.0] },
            Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal: [0.0, 1.0, 0.0] },

            // Bottom face
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [0.0, -1.0, 0.0] },
            Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal: [0.0, -1.0, 0.0] },
            Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal: [0.0, -1.0, 0.0] },
            Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal: [0.0, -1.0, 0.0] },
            Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal: [0.0, -1.0, 0.0] },
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [0.0, -1.0, 0.0] },

            // Right face
            Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal: [1.0, 0.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal: [1.0, 0.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [1.0, 0.0, 0.0] },
            Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal: [1.0, 0.0, 0.0] },
            Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal: [1.0, 0.0, 0.0] },
            Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal: [1.0, 0.0, 0.0] },

            // Left face
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal: [-1.0, 0.0, 0.0] }
        ]
    }
}
