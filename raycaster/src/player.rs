use crate::math::Vec2;
use crate::camera::Camera;
use crate::world::World;

pub struct Player {
    pos: Vec2,
    camera: Camera,
    move_speed: f32,
    rot_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        let pos = Vec2::default();

        Self {
            pos: pos.clone(),
            camera: Camera::new(pos),
            move_speed: 3.0,
            rot_speed: 5.0,
        }
    }

    pub fn init(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn move_forward(&mut self, world: &World, delta_time: f64) {
        self.camera.move_forward(world, self.move_speed, delta_time);
    }

    pub fn move_backwards(&mut self, world: &World, delta_time: f64) {
        self.camera.move_backward(world, self.move_speed, delta_time);
    }

    pub fn rotate_left(&mut self, delta_time: f64) {
        self.camera.rotate_left(self.rot_speed, delta_time);
    }

    pub fn rotate_right(&mut self, delta_time: f64) {
        self.camera.rotate_right(self.rot_speed, delta_time);
    }
}