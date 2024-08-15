use crate::renderer::Renderer;
use crate::world::World;

pub struct Raycaster {
    time        : u128,

    renderer    : Renderer,
    world       : World,
}

impl Raycaster {
    pub fn new() -> Self {
        Self {
            time: 0,
            renderer: Renderer::new(),
            world: World::new(),
        }
    }

    pub fn run(&self) {
        self.renderer.render();
    }
}