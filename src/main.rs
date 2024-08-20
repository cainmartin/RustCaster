use raycaster::prelude::Raycaster;
use raycaster::prelude::MapData;
use raycaster::prelude::utilities::load_json;

fn main() {
    println!("Initializing rust_caster");

    let mut rc = Raycaster::new();

    let file_path = "assets/maps/test_map.json";
    match load_json::<MapData>(file_path) {
        Ok(map_data) => {
            println!("Map data loaded");
            rc.init(&map_data);
        }
        Err(e) => {
            eprintln!("Failed to load map data: {}", e);
        }
    }

    rc.run();
}
