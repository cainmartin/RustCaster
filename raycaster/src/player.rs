use crate::math::Vec2;
use crate::camera::Camera;

pub struct Player {
    pos: Vec2,
    camera: Camera,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::default(),
            camera: Camera::new(),
        }
    }
}