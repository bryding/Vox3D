use noise::{ NoiseFn, Perlin };

pub struct TerrainGenerator {
    noise: Perlin,
    height_scale: f64,
    noise_scale: f64,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new(seed),
            height_scale: 32.0, // Max height of terrain
            noise_scale: 0.02, // Adjust this to change terrain frequency
        }
    }

    pub fn get_height(&self, x: i32, z: i32) -> i32 {
        let noise_value = self.noise.get([
            (x as f64) * self.noise_scale,
            (z as f64) * self.noise_scale,
        ]);

        // Convert noise from [-1, 1] to [0, height_scale]
        let height = ((noise_value + 1.0) * 0.5 * self.height_scale) as i32;

        // Ensure minimum height of 1 block
        height.max(1)
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Vec<bool> {
        let chunk_size = 16;
        let max_height = 256;
        let mut voxels = vec![false; (chunk_size * chunk_size * max_height) as usize];

        for x in 0..chunk_size {
            for z in 0..chunk_size {
                let world_x = chunk_x * chunk_size + x;
                let world_z = chunk_z * chunk_size + z;

                // Clamp height to max_height - 1 to avoid out-of-bounds access
                let height = self.get_height(world_x, world_z).min(max_height - 1);

                // Fill all blocks from 0 to height
                for y in 0..=height {
                    let index = x + z * chunk_size + y * chunk_size * chunk_size;
                    voxels[index as usize] = true;
                }
            }
        }
        voxels
    }
}
