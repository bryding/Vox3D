use cgmath::{ Point3, Vector3 };
use winit::event::*;
use std::collections::{ HashMap, HashSet };

use crate::terrain::generator::TerrainGenerator;
use crate::terrain::chunk::Chunk;
use super::camera_controller::CameraController;

pub struct GameState {
    camera_controller: CameraController,
    camera_position: Point3<f32>,
    camera_direction: Vector3<f32>,
    camera_up: Vector3<f32>,
    terrain_generator: TerrainGenerator,
    render_distance: i32,
    chunks: HashMap<(i32, i32), Chunk>,
    loaded_chunks: HashSet<(i32, i32)>, // Track which chunks are currently loaded
    chunks_updated: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            camera_controller: CameraController::new(100.0, 0.1),
            camera_position: Point3::new(0.0, 70.0, 0.0),
            camera_direction: Vector3::new(0.0, -0.5, -1.0),
            camera_up: Vector3::new(0.0, 1.0, 0.0),
            terrain_generator: TerrainGenerator::new(42),
            chunks: HashMap::new(),
            render_distance: 6, // Number of chunks to load in each direction
            loaded_chunks: HashSet::new(),
            chunks_updated: false,
        };

        // Generate initial chunks
        state.update_chunks();
        state
    }

    pub fn update_chunks(&mut self) -> bool {
        let chunks_in_range = self.get_chunks_in_range();

        // Find chunks to unload
        let chunks_to_unload: Vec<(i32, i32)> = self.loaded_chunks
            .difference(&chunks_in_range)
            .cloned()
            .collect();

        // Find chunks to load
        let chunks_to_load: Vec<(i32, i32)> = chunks_in_range
            .difference(&self.loaded_chunks)
            .cloned()
            .collect();

        // Store the lengths before we move the vectors
        let has_changes = !chunks_to_load.is_empty() || !chunks_to_unload.is_empty();

        // Unload chunks
        for chunk_pos in chunks_to_unload {
            self.chunks.remove(&chunk_pos);
            self.loaded_chunks.remove(&chunk_pos);
        }

        // Load new chunks
        for chunk_pos in chunks_to_load {
            let voxels = self.terrain_generator.generate_chunk(chunk_pos.0, chunk_pos.1);
            self.chunks.insert(chunk_pos, Chunk::new(voxels, chunk_pos.0, chunk_pos.1));
            self.loaded_chunks.insert(chunk_pos);
        }

        self.chunks_updated = has_changes;
        has_changes
    }

    // Get the chunks that should be loaded based on render distance
    fn get_chunks_in_range(&self) -> HashSet<(i32, i32)> {
        let (center_x, center_z) = self.get_current_chunk();
        let mut chunks = HashSet::new();

        for x in -self.render_distance..=self.render_distance {
            for z in -self.render_distance..=self.render_distance {
                chunks.insert((center_x + x, center_z + z));
            }
        }
        chunks
    }

    pub fn chunks_updated(&self) -> bool {
        self.chunks_updated
    }

    // Convert camera position to chunk coordinates
    fn get_current_chunk(&self) -> (i32, i32) {
        let chunk_size = 16; // Must match the value in TerrainGenerator
        (
            (self.camera_position.x / (chunk_size as f32)).floor() as i32,
            (self.camera_position.z / (chunk_size as f32)).floor() as i32,
        )
    }

    pub fn handle_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        self.camera_controller.process_keyboard(key, state)
    }

    pub fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        self.camera_controller.process_mouse(dx, dy);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.chunks_updated = false;
        // Update camera
        self.camera_controller.update_camera(
            &mut self.camera_position,
            &mut self.camera_direction,
            &mut self.camera_up,
            dt
        );

        // Update chunks based on new camera position
        self.update_chunks();
    }

    pub fn chunks(&self) -> &HashMap<(i32, i32), Chunk> {
        &self.chunks
    }

    pub fn camera_position(&self) -> Point3<f32> {
        self.camera_position
    }

    pub fn camera_direction(&self) -> Vector3<f32> {
        self.camera_direction
    }

    pub fn camera_up(&self) -> Vector3<f32> {
        self.camera_up
    }
}
