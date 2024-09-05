use crate::defines::*;
use crate::color::*;

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
        // This is currently not useful as we fill the buffer in draw_line
        // self.clear_color();
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    fn clear_color(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0x000000;
        }
    }

    pub fn draw_line(&mut self, col: i32, draw_start: i32, draw_end: i32, color: &ColorRGB) {
        for row in draw_start..draw_end {
            let index = (row * (WIDTH as i32) + col) as usize;
            self.buffer[index] = color.to_hex();
        }
    }
}