
pub struct Camera {
    dir: Vec2,
    plane: Vec2,
    fov: f32,

}

impl Camera {
    pub fn new() -> Self {
        Self {
            dir: { -1.0, 0.0 },
            plane: { 0.0, 0.66 }
            fov: 60.0,
        }
    }

    pub fn move_forwards(&self, speed: f32, delta: f32) {

    }

    pub fn move_backwards(&self, speed: f32, delta: f32) {

    }

    pub fn rotate_left(&self, rot_speed: f32, delta: f32) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * rot_speed.cos() - self.dir.y * rot_speed.sin();
        self.dir.y = old_dir_x * rot_speed.sin() + self.dir.y * rot_speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * rot_speed.cos() - self.plane.y * rot_speed.sin();
        self.plane.y = old_plane_x * rot_speed.sin() + self.plane.y * rot_speed.cos();
    }

    pub fn rotate_right(&self, rot_speed: f32, delta: f32) {
        // Rotate the direction vector - this is effectively an imperative version of a matrix multiplication
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * -rot_speed.cos() - self.dir.y * -rot_speed.sin();
        self.dir.y = old_dir_x * -rot_speed.sin() + self.dir.y * -rot_speed.cos();

        // Rotate the plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * -rot_speed.cos() - self.plane.y * -rot_speed.sin();
        self.plane.y = old_plane_x * -rot_speed.sin() + self.plane.y * -rot_speed.cos();
    }
}