use crate::renderer::Renderer;
use crate::world::World;

pub struct Raycaster {
    time        : f64,
    old_time    : f64,
    renderer    : Renderer,
    world       : World,
}

impl Raycaster {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            old_time: 0.0,
            renderer: Renderer::new(),
            world: World::new(),
        }
    }

    pub fn run(&self) {
        self.renderer.render();
    }
}