use cgmath::{ Point3, Vector3 };
use winit::event::*;
use std::collections::HashMap;

use crate::terrain::generator::TerrainGenerator;
use crate::terrain::chunk::Chunk;
use super::camera_controller::CameraController;

pub struct GameState {
    camera_controller: CameraController,
    camera_position: Point3<f32>,
    camera_direction: Vector3<f32>,
    camera_up: Vector3<f32>,
    terrain_generator: TerrainGenerator,
    chunks: HashMap<(i32, i32), Chunk>,
}

impl GameState {
    pub fn new() -> Self {
        let terrain_generator = TerrainGenerator::new(42); // Using seed 42
        let mut chunks = HashMap::new();

        // Generate a 5x5 area of chunks centered on (0,0)
        for chunk_x in -2..=2 {
            for chunk_z in -2..=2 {
                let voxels = terrain_generator.generate_chunk(chunk_x, chunk_z);
                chunks.insert((chunk_x, chunk_z), Chunk::new(voxels, chunk_x, chunk_z));
            }
        }

        Self {
            camera_controller: CameraController::new(100.0, 0.1),
            camera_position: Point3::new(0.0, 70.0, 0.0), // Start high up to see terrain
            camera_direction: Vector3::new(0.0, -0.5, -1.0), // Look down and forward
            camera_up: Vector3::new(0.0, 1.0, 0.0),
            terrain_generator,
            chunks,
        }
    }

    pub fn handle_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        self.camera_controller.process_keyboard(key, state)
    }

    pub fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        self.camera_controller.process_mouse(dx, dy);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_camera(
            &mut self.camera_position,
            &mut self.camera_direction,
            &mut self.camera_up,
            dt
        );
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
