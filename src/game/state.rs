use cgmath::{ Point3, Vector3 };
use winit::event::*;

use super::camera_controller::CameraController;

pub struct GameState {
    camera_controller: CameraController,
    camera_position: Point3<f32>,
    camera_direction: Vector3<f32>,
    camera_up: Vector3<f32>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera_controller: CameraController::new(5.0, 0.01),
            camera_position: Point3::new(0.0, 0.0, 10.0),
            camera_direction: Vector3::new(0.0, 0.0, -1.0),
            camera_up: Vector3::new(0.0, 1.0, 0.0),
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

    // Getters for render system
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
