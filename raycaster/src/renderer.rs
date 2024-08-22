use crate::defines::*;

pub struct Renderer {
    buffer: Vec<u32>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; WIDTH * HEIGHT]
        }
    }

    // TODO: split up the render function so that we can tick this from the game engine
    pub fn render(&mut self, _delta_time: f64) {
        self.clear_color();
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    fn clear_color(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0xFF0000;
        }
    }
}