
pub struct Camera {
    dir: Vec2,
    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            dir: Vec2::default(),
            fov: 60.0,
        }
    }
}