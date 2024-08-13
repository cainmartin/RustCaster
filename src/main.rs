use raycaster::prelude::Raycaster;

fn main() {
    println!("Initializing rust_caster");

    let rc = Raycaster::new();
    rc.run();
}
