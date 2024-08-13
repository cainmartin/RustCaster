use crate::math::Vec2;

pub struct Player {
    pos: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::default()
        }
    }
}