use noise::{NoiseFn, Perlin};

pub struct TerrainGenerator {
    noise: Perlin,
    height_scale: f64,
    noise_scale: f64,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new(seed),
            height_scale: 32.0,
            noise_scale: 0.01,
        }
    }

    pub fn get_height(&self, x: i32, z: i32) -> i32 {
        let noise_value = self
            .noise
            .get([x as f64 * self.noise_scale, z as f64 * self.noise_scale]);

        // Convert noise from [-1, 1] to [0, height_scale]
        ((noise_value + 1.0) * 0.5 * self.height_scale) as i32
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Vec<bool> {
        let chunk_size: i32 = 16;
        let max_height: i32 = 256;

        let size = (chunk_size * chunk_size * max_height) as usize;
        let mut voxels = vec![false; size];

        for x in 0..chunk_size {
            for z in 0..chunk_size {
                let world_x = chunk_x * chunk_size + x;
                let world_z = chunk_z * chunk_size + z;
                let height = self.get_height(world_x, world_z);

                for y in 0..height {
                    let x_usize = x as usize;
                    let y_usize = y as usize;
                    let z_usize = z as usize;
                    let chunk_size_usize = chunk_size as usize;

                    let index = x_usize
                        + z_usize * chunk_size_usize
                        + y_usize * chunk_size_usize * chunk_size_usize;

                    voxels[index] = true;
                }
            }
        }

        voxels
    }
}
