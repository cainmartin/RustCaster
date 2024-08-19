use crate::defines::*;
use minifb::{Key, Window, WindowOptions };

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Self {
        Self {

        }
    }

    // TODO: split up the render function so that we can tick this from the game engine
    pub fn render(&self) {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let mut window = Window::new(
            "Rustenstein",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e)
        });
    
        window.set_target_fps(FPS);
    
        while window.is_open() && !window.is_key_down(Key::Escape) {
    
            for iter in buffer.iter_mut() {
                *iter = 0;
            }
    
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}