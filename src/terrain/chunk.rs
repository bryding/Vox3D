pub struct Chunk {
    pub voxels: Vec<bool>,
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Chunk {
    pub fn new(voxels: Vec<bool>, chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            voxels,
            chunk_x,
            chunk_z,
        }
    }
}
