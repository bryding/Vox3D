use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 10.0),   // Start 10 units back
            direction: Vector3::new(0.0, 0.0, -1.0), // Look forward
            up: Vector3::new(0.0, 1.0, 0.0),         // Y is up
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        // Create view matrix from camera position and direction
        let view = Matrix4::look_to_rh(self.position, self.direction, self.up);

        // Create projection matrix
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // Combine them
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

// We need this because wgpu's coordinate system is based on DirectX/Metal
// while cgmath uses OpenGL's coordinate system
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
