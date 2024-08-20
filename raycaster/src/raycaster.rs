use crate::renderer::Renderer;
use crate::world::World;
use crate::player::Player;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PlayerStart {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug)]
struct MapSize {
    width: i32,
    height: i32,
}

#[derive(Deserialize, Debug)]
pub struct MapData {
    player_start: PlayerStart,
    map_size: MapSize,
    map_data: Vec<String>,
}

pub struct Raycaster {
    time        : f64,
    old_time    : f64,
    renderer    : Renderer,
    player      : Player,
    world       : World,
}

impl Raycaster {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            old_time: 0.0,
            renderer: Renderer::new(),
            player: Player::new(),
            world: World::new(),
        }
    }

    pub fn init(&mut self, map_data: &MapData) {
        let width = map_data.map_size.width;
        let height = map_data.map_size.height;
        let map: Vec<u8> = map_data
            .map_data
            .iter()
            .flat_map(|s| s.as_bytes().to_vec())
            .collect();

        self.world.init(width, height, map);
    }

    pub fn run(&self) {
        self.renderer.render();
    }
}