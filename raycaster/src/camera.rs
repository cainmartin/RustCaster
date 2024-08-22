use crate::world::World;
use crate::math::Vec2;

pub struct Camera {
    pos: Vec2,
    dir: Vec2,
    plane: Vec2,
    _fov: f32,
}

impl Camera {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos: pos,
            dir: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.66),
            _fov: 60.0,
        }
    }

    pub fn move_forward(&mut self, world: &World, speed: f32, delta_time: f64) {
        let move_speed = speed * delta_time as f32;

        if world.is_collision((self.pos.x + self.dir.x * move_speed) as i32, self.pos.y as i32) == false {
            self.pos.x += self.dir.x * move_speed;
        }

        if world.is_collision(self.pos.x as i32, (self.pos.y + self.dir.y * move_speed) as i32) == false {
            self.pos.y += self.dir.y * move_speed;
        }
    }

    pub fn move_backward(&mut self, world: &World, speed: f32, delta_time: f64) {
        let move_speed = speed * delta_time as f32;

        if world.is_collision((self.pos.x - self.dir.x * move_speed) as i32, self.pos.y as i32) == false {
            self.pos.x -= self.dir.x * move_speed;
        }

        if world.is_collision(self.pos.x as i32, (self.pos.y - self.dir.y * move_speed) as i32) == false {
            self.pos.y -= self.dir.y * move_speed;
        }
    }

    pub fn rotate_left(&mut self, rot_speed: f32, delta_time: f64) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = rot_speed * delta_time as f32;
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * speed.cos() - self.dir.y * speed.sin();
        self.dir.y = old_dir_x * speed.sin() + self.dir.y * speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * speed.cos() - self.plane.y * speed.sin();
        self.plane.y = old_plane_x * speed.sin() + self.plane.y * speed.cos();
    }

    pub fn rotate_right(&mut self, rot_speed: f32, delta_time: f64) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = rot_speed * delta_time as f32;
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * -speed.cos() - self.dir.y * -speed.sin();
        self.dir.y = old_dir_x * -speed.sin() + self.dir.y * -speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * -speed.cos() - self.plane.y * -speed.sin();
        self.plane.y = old_plane_x * -speed.sin() + self.plane.y * -speed.cos();
    }
}