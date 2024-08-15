use raycaster::prelude::Raycaster;
use raycaster::prelude::utilities::load_json;
use raycaster::prelude::math::Vec2;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PlayerStart {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug)]
struct MapData {
    player_start: PlayerStart,
    map_data: Vec<String>,
}

fn main() {
    println!("Initializing rust_caster");

    let file_path = "assets/maps/test_map.json";
    match load_json::<MapData>(file_path) {
        Ok(map_data) => {
            println!("{:?}", map_data);
        }
        Err(e) => {
            eprintln!("Faied to load map data: {}", e);
        }
    }

    let rc = Raycaster::new();
    rc.run();
}
