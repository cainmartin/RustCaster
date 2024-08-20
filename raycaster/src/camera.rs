use crate::world::World;
use crate::math::Vec2;

pub struct Camera {
    dir: Vec2,
    plane: Vec2,
    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            dir: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.66),
            fov: 60.0,
        }
    }

    pub fn move_forwards(&mut self, world: &World, speed: f32, delta: f32) {
        
    }

    pub fn move_backwards(&mut self, world: &World, speed: f32, delta: f32) {
       
    }

    pub fn rotate_left(&mut self, rot_speed: f32, delta: f32) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = rot_speed * delta;
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * speed.cos() - self.dir.y * speed.sin();
        self.dir.y = old_dir_x * speed.sin() + self.dir.y * speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * speed.cos() - self.plane.y * speed.sin();
        self.plane.y = old_plane_x * speed.sin() + self.plane.y * speed.cos();
    }

    pub fn rotate_right(&mut self, rot_speed: f32, delta: f32) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = rot_speed * delta;
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * -speed.cos() - self.dir.y * -speed.sin();
        self.dir.y = old_dir_x * -speed.sin() + self.dir.y * -speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * -speed.cos() - self.plane.y * -speed.sin();
        self.plane.y = old_plane_x * -speed.sin() + self.plane.y * -speed.cos();
    }
}