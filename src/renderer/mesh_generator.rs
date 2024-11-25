use super::vertex::Vertex;

pub struct MeshGenerator;

impl MeshGenerator {
    // Create a single face of a cube so we can optimize for not drawing every face
    fn create_face(pos: [f32; 3], normal: [f32; 3], color: [f32; 3]) -> Vec<Vertex> {
        let (x, y, z) = (pos[0], pos[1], pos[2]);

        match normal {
            [0.0, 0.0, 1.0] =>
                vec![
                    // Front face
                    Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal }
                ],
            [0.0, 0.0, -1.0] =>
                vec![
                    // Back face
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal }
                ],
            [0.0, 1.0, 0.0] =>
                vec![
                    // Top face
                    Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal }
                ],
            [0.0, -1.0, 0.0] =>
                vec![
                    // Bottom face
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal }
                ],
            [1.0, 0.0, 0.0] =>
                vec![
                    // Right face
                    Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x + 0.5, y - 0.5, z - 0.5], color, normal }
                ],
            [-1.0, 0.0, 0.0] =>
                vec![
                    // Left face
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z + 0.5], color, normal },
                    Vertex { position: [x - 0.5, y + 0.5, z - 0.5], color, normal },
                    Vertex { position: [x - 0.5, y - 0.5, z - 0.5], color, normal }
                ],
            _ => vec![], // Invalid normal
        }
    }

    pub fn generate_chunk_mesh(voxels: &[bool], chunk_x: i32, chunk_z: i32) -> Vec<Vertex> {
        let chunk_size = 16;
        let max_height = 256;
        let mut vertices = Vec::new();

        let get_voxel = |x: i32, y: i32, z: i32| -> bool {
            if x < 0 || x >= chunk_size || y < 0 || y >= max_height || z < 0 || z >= chunk_size {
                return false;
            }
            let index = (x + z * chunk_size + y * chunk_size * chunk_size) as usize;
            voxels[index]
        };

        for x in 0..chunk_size {
            for y in 0..max_height {
                for z in 0..chunk_size {
                    if !get_voxel(x, y, z) {
                        continue;
                    }

                    let world_x = chunk_x * chunk_size + x;
                    let world_z = chunk_z * chunk_size + z;

                    let color = if y < 5 {
                        [0.7, 0.7, 0.3] // Sand
                    } else if y < 12 {
                        [0.3, 0.5, 0.2] // Grass
                    } else {
                        [0.5, 0.5, 0.5] // Stone
                    };

                    let pos = [world_x as f32, y as f32, world_z as f32];

                    // Add faces only if adjacent block is air
                    if !get_voxel(x, y, z + 1) {
                        vertices.extend(Self::create_face(pos, [0.0, 0.0, 1.0], color));
                    }
                    if !get_voxel(x, y, z - 1) {
                        vertices.extend(Self::create_face(pos, [0.0, 0.0, -1.0], color));
                    }
                    if !get_voxel(x, y + 1, z) {
                        vertices.extend(Self::create_face(pos, [0.0, 1.0, 0.0], color));
                    }
                    if !get_voxel(x, y - 1, z) {
                        vertices.extend(Self::create_face(pos, [0.0, -1.0, 0.0], color));
                    }
                    if !get_voxel(x + 1, y, z) {
                        vertices.extend(Self::create_face(pos, [1.0, 0.0, 0.0], color));
                    }
                    if !get_voxel(x - 1, y, z) {
                        vertices.extend(Self::create_face(pos, [-1.0, 0.0, 0.0], color));
                    }
                }
            }
        }

        vertices
    }
}
