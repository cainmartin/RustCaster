
pub struct World {
    width: i32,
    height: i32,
}

impl World {
    pub fn new() -> Self {
        Self {
            width: 20,
            height: 20,
        }
    }
}