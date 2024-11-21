use cgmath::{ Point3, Vector3, InnerSpace };
use winit::event::*;
use std::time::Duration;

pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
    yaw: f32,
    pitch: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let is_pressed = state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.is_forward_pressed = is_pressed;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.is_backward_pressed = is_pressed;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.is_left_pressed = is_pressed;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.is_right_pressed = is_pressed;
                true
            }
            VirtualKeyCode::Space => {
                self.is_up_pressed = is_pressed;
                true
            }
            VirtualKeyCode::LShift => {
                self.is_down_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, dx: f64, dy: f64) {
        self.yaw += (dx as f32) * self.sensitivity;
        self.pitch -= (dy as f32) * self.sensitivity;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }

    pub fn update_camera(
        &self,
        position: &mut Point3<f32>,
        direction: &mut Vector3<f32>,
        up: &mut Vector3<f32>,
        dt: Duration
    ) {
        // Calculate the new front vector
        let yaw_radians = self.yaw.to_radians();
        let pitch_radians = self.pitch.to_radians();

        *direction = Vector3::new(
            yaw_radians.cos() * pitch_radians.cos(),
            pitch_radians.sin(),
            yaw_radians.sin() * pitch_radians.cos()
        ).normalize();

        // Calculate the right and up vectors
        let right = direction.cross(Vector3::new(0.0, 1.0, 0.0)).normalize();
        *up = right.cross(*direction).normalize();

        // Scale movement by delta time
        let movement_speed = self.speed * dt.as_secs_f32();
        let forward = *direction * movement_speed;
        let right = right * movement_speed;

        if self.is_forward_pressed {
            *position += forward;
        }
        if self.is_backward_pressed {
            *position -= forward;
        }
        if self.is_right_pressed {
            *position += right;
        }
        if self.is_left_pressed {
            *position -= right;
        }
        if self.is_up_pressed {
            position.y += movement_speed;
        }
        if self.is_down_pressed {
            position.y -= movement_speed;
        }
    }
}
