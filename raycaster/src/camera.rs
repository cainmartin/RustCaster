use crate::world::World;
use crate::math::Vec2;

pub struct Camera {
    pub(crate) pos: Vec2,
    pub(crate) dir: Vec2,
    pub(crate) plane: Vec2,
    _fov: f64,
    _move_speed: f64,
    _rotation_speed: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            dir: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.66),
            _fov: 60.0,
            _move_speed: 3.0,
            _rotation_speed: 5.0,
        }
    }

    pub fn init(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn move_forward(&mut self, world: &World, delta_time: f64) {
        let move_speed = self._move_speed * delta_time;

        if world.is_collision((self.pos.x + self.dir.x * move_speed) as i64, self.pos.y as i64) == false {
            self.pos.x += self.dir.x * move_speed;
        }

        if world.is_collision(self.pos.x as i64, (self.pos.y + self.dir.y * move_speed) as i64) == false {
            self.pos.y += self.dir.y * move_speed;
        }
    }

    pub fn move_backward(&mut self, world: &World, delta_time: f64) {
        let move_speed = self._move_speed * delta_time;

        if world.is_collision((self.pos.x - self.dir.x * move_speed) as i64, self.pos.y as i64) == false {
            self.pos.x -= self.dir.x * move_speed;
        }

        if world.is_collision(self.pos.x as i64, (self.pos.y - self.dir.y * move_speed) as i64) == false {
            self.pos.y -= self.dir.y * move_speed;
        }
    }

    pub fn rotate_left(&mut self, delta_time: f64) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = self._rotation_speed * delta_time;

        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * speed.cos() - self.dir.y * speed.sin();
        self.dir.y = old_dir_x * speed.sin() + self.dir.y * speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * speed.cos() - self.plane.y * speed.sin();
        self.plane.y = old_plane_x * speed.sin() + self.plane.y * speed.cos();
    }

    pub fn rotate_right(&mut self, delta_time: f64) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let speed = self._rotation_speed * delta_time;

        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * -speed.cos() - self.dir.y * -speed.sin();
        self.dir.y = old_dir_x * -speed.sin() + self.dir.y * -speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * -speed.cos() - self.plane.y * -speed.sin();
        self.plane.y = old_plane_x * -speed.sin() + self.plane.y * -speed.cos();
    }
}