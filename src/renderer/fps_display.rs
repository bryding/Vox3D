use std::time::Instant;

pub struct FpsDisplay {
    fps_values: Vec<u32>,
    last_frame: Instant,
    fps_update_timer: Instant,
    current_fps: u32,
}

impl FpsDisplay {
    pub fn new() -> Self {
        Self {
            fps_values: Vec::with_capacity(10),
            last_frame: Instant::now(),
            fps_update_timer: Instant::now(),
            current_fps: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);
        self.last_frame = now;

        let fps = if frame_time.as_secs_f32() > 0.0 {
            (1.0 / frame_time.as_secs_f32()) as u32
        } else {
            0
        };

        self.fps_values.push(fps);

        if self.fps_update_timer.elapsed() > std::time::Duration::from_millis(100) {
            if self.fps_values.len() > 10 {
                self.fps_values.remove(0);
            }
            self.fps_update_timer = Instant::now();
        }

        self.current_fps = if !self.fps_values.is_empty() {
            self.fps_values.iter().sum::<u32>() / (self.fps_values.len() as u32)
        } else {
            0
        };
    }

    pub fn fps(&self) -> u32 {
        self.current_fps
    }
}
