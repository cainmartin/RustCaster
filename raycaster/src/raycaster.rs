use crate::renderer::Renderer;

pub struct Raycaster {
    time        : u128,
}

impl Raycaster {
    pub fn new() -> Self {
        Self {
            time: 0,
        }
    }

    pub fn run(&self) {
        let renderer = Renderer::new();
        renderer.render();
    }
}