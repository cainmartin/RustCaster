
#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}