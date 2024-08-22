use crate::renderer::Renderer;
use crate::world::World;
use crate::player::Player;
use serde::Deserialize;
use minifb::{Key, Window, WindowOptions };
use crate::defines::*;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct PlayerStart {
    x: f32,
    y: f32,
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
    last_time   : Instant,
    renderer    : Renderer,
    window      : Window,
    player      : Player,
    world       : World,
}

impl Raycaster {
    pub fn new() -> Self {

        let window = Window::new(
            "Rustenstein",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e)
        });

        Self {
            last_time: Instant::now(),
            renderer: Renderer::new(),
            window,
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
        self.player.init(map_data.player_start.x, map_data.player_start.y);
    }

    pub fn run(&mut self) {
        self.window.set_target_fps(FPS);
        
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let delta_time = now.duration_since(self.last_time).as_secs_f64();
            self.last_time = now;

            self.handle_input(delta_time);
            self.update(delta_time);
            self.render(delta_time);
        }

    }

    pub fn update(&self, _delta_time: f64) {

    }

    pub fn handle_input(&mut self, delta_time: f64) {
        if self.window.is_key_down(Key::W) {
            self.player.move_forward(&self.world, delta_time);
        }

        if self.window.is_key_down(Key::S) {
            self.player.move_backwards(&self.world, delta_time);
        }

        if self.window.is_key_down(Key::A) {
            self.player.rotate_left(delta_time);
        }

        if self.window.is_key_down(Key::D) {
            self.player.rotate_right(delta_time);
        }
    }

    pub fn render(&mut self, delta_time: f64) {
        self.renderer.render(delta_time);
        let buffer = self.renderer.get_buffer();

        self.window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}