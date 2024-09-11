use crate::renderer::Renderer;
use crate::world::World;
use crate::camera::Camera;
use serde::Deserialize;
use minifb::{Key, Window, WindowOptions };
use crate::defines::*;
use crate::color::*;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct PlayerStart {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Debug)]
struct MapSize {
    width: i64,
    height: i64,
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
    camera      : Camera,
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
            camera: Camera::new(),
            world: World::new(),
        }
    }

    pub fn init(&mut self, map_data: &MapData) {
        let width = map_data.map_size.width;
        let height = map_data.map_size.height;
        let map: Vec<u8> = map_data
            .map_data
            .iter()
            .flat_map(|s| s.chars().filter_map(|c| match c {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                _ => None,
            }))
            .collect();

        self.world.init(width, height, map);
        self.camera.init(map_data.player_start.x, map_data.player_start.y);
    }

    pub fn run(&mut self) {
        self.window.set_target_fps(FPS);
        
        let mut pos_x: f64 = 20.0;
        let mut pos_y: f64 = 20.0;
        let mut dir_x: f64  = -1.0;
        let mut dir_y: f64 = 0.0;
        let mut plane_x: f64 = 0.0;
        let mut plane_y: f64 = 0.66;

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let delta_time = now.duration_since(self.last_time).as_secs_f64();
            self.last_time = now;

            // Get the screen rendering here for now - we will plug in the camera world later

            self.renderer.clear_color();

            for x in 0..WIDTH {
                let camera_x = 2.0 * (x as f64) / (WIDTH as f64) - 1.0;
                let ray_dir_x = dir_x + plane_x * camera_x;
                let ray_dir_y = dir_y + plane_y * camera_x;

                // Calculate the actual box of the map we are in
                let mut map_x = pos_x as i64;
                let mut map_y = pos_y as i64;

                // Length of the ray from the current position, to next x or y-side
                let mut side_dist_x = 0.0;
                let mut side_dist_y = 0.0;

                //length of ray from one x or y-side to next x or y-side
                //these are derived as:
                //deltaDistX = sqrt(1 + (rayDirY * rayDirY) / (rayDirX * rayDirX))
                //deltaDistY = sqrt(1 + (rayDirX * rayDirX) / (rayDirY * rayDirY))
                //which can be simplified to abs(|rayDir| / rayDirX) and abs(|rayDir| / rayDirY)
                //where |rayDir| is the length of the vector (rayDirX, rayDirY). Its length,
                //unlike (dirX, dirY) is not 1, however this does not matter, only the
                //ratio between deltaDistX and deltaDistY matters, due to the way the DDA
                //stepping further below works. So the values can be computed as below.
                // Division through zero is prevented, even though technically that's not
                // needed in C++ with IEEE 754 floating point values.

                let mut delta_dist_x = if ray_dir_x == 0.0 {
                    1e30
                } else {
                    (1.0 / ray_dir_x).abs() as f64
                };

                let mut delta_dist_y = if ray_dir_y == 0.0 { 
                    1e30 
                } else { 
                    (1.0 / ray_dir_y).abs() as f64
                };
                
                let mut perp_wall_dist = 0.0;

                let mut step_x : i64 = 0;
                let mut step_y: i64 = 0;

                let mut hit:i64 = 0; // Was there a hit
                let mut side:i64 = 0; // Was a NS or EW wall hit?

                // Calculate step and initial side_dist
                if ray_dir_x < 0.0 {
                    step_x = -1;
                    // TODO: delta_dist_x is f64 - need to verify this
                    side_dist_x = (pos_x - (map_x as f64)) * delta_dist_x;
                } else {
                    step_x = 1;
                    side_dist_x = ((map_x as f64) + 1.0 - pos_x) * delta_dist_x;
                }

                if ray_dir_y < 0.0 {
                    step_y = -1;
                    side_dist_y = (pos_y - (map_y as f64)) * (delta_dist_y);
                } else {
                    step_y = 1;
                    side_dist_y = ((map_y as f64) + 1.0 - pos_y) * (delta_dist_y);
                }


                while hit == 0 {
                    if side_dist_x < side_dist_y {
                        side_dist_x = side_dist_x + delta_dist_x;
                        map_x = map_x + step_x;
                        side = 0;
                    }
                    else
                    {
                        side_dist_y = side_dist_y + delta_dist_y;
                        map_y = map_y + step_y;
                        side = 1;
                    }

                    if self.world.is_collision(map_x as i64, map_y as i64) {
                        hit = 1;
                    }
                }

                // Calculate distance projected on camera direction. This is the shortest distance from the point where the wall is
                // hit to the camera plane. Euclidean to center camera point would give fisheye effect!
                // This can be computed as (mapX - posX + (1 - stepX) / 2) / rayDirX for side == 0, or same formula with Y
                // for size == 1, but can be simplified to the code below thanks to how sideDist and deltaDist are computed:
                // because they were left scaled to |rayDir|. sideDist is the entire length of the ray above after the multiple
                // steps, but we subtract deltaDist once because one step more into the wall was taken above.

                if side == 0 {
                    perp_wall_dist = side_dist_x - delta_dist_x;
                } else {
                    perp_wall_dist = side_dist_y - delta_dist_y;
                }

                // Calculate height of line to draw on screen
                let line_height = ((HEIGHT as f64) / perp_wall_dist) as i32;

                // calculate lowest and highest pixel to fill in current stripe
                let mut draw_start = -line_height / 2 + (HEIGHT as i32) / 2;
                if draw_start < 0 {
                    draw_start = 0;
                }

                let mut draw_end = line_height / 2 + (HEIGHT as i32) / 2;
                if draw_end >= (HEIGHT as i32) {
                    draw_end = (HEIGHT as i32) - 1;
                }

                let mut color = match self.world.get_cell(map_x as i64, map_y as i64) {
                    1 => RED_RGB,
                    2 => GREEN_RGB,
                    3 => BLUE_RGB,
                    4 => WHITE_RGB,
                    _ => YELLOW_RGB,
                };

                if side == 1 {
                    color = color / 2;
                }

                self.renderer.draw_line(x as i32, draw_start, draw_end, &color);
            } // Draw screen

            // TEMP MOVEMENT CODE - REMOVE

            let move_speed = 5.0 * delta_time;
            let rotate_speed = 3.0 * delta_time;

            if self.window.is_key_down(Key::W) {
                if self.world.is_collision((pos_x + dir_x) as i64, pos_y as i64) == false {
                    pos_x += dir_x * move_speed;
                }
                if self.world.is_collision(pos_x as i64, (pos_y + dir_y) as i64) == false {
                    pos_y += dir_y * move_speed;
                }
            }

            if self.window.is_key_down(Key::S) {
                if self.world.is_collision((pos_x - dir_x) as i64, pos_y as i64) == false {
                    pos_x -= dir_x * move_speed;
                }
                if self.world.is_collision(pos_x as i64, (pos_y - dir_y) as i64) == false {
                    pos_y -= dir_y * move_speed;
                }
            }

            if self.window.is_key_down(Key::D) {
                let old_dir_x = dir_x;
                dir_x = dir_x * (-rotate_speed).cos() - dir_y * (-rotate_speed).sin();
                dir_y = old_dir_x * (-rotate_speed).sin() + dir_y * (-rotate_speed).cos();
                let old_plane_x = plane_x;
                plane_x = plane_x * (-rotate_speed).cos() - plane_y * (-rotate_speed).sin();
                plane_y = old_plane_x * (-rotate_speed).sin() + plane_y * (-rotate_speed).cos();
            }

            if self.window.is_key_down(Key::A) {
                let old_dir_x = dir_x;
                dir_x = dir_x * rotate_speed.cos() - dir_y * rotate_speed.sin();
                dir_y = old_dir_x * rotate_speed.sin() + dir_y * rotate_speed.cos();
                let old_plane_x = plane_x;
                plane_x = plane_x * rotate_speed.cos() - plane_y * rotate_speed.sin();
                plane_y = old_plane_x * rotate_speed.sin() + plane_y * rotate_speed.cos();
            }

            // 
            self.handle_input(delta_time);
            self.update(delta_time);
            self.render(delta_time);

        } // Main loop
    } // run

    pub fn update(&self, _delta_time: f64) {

    }

    pub fn handle_input(&mut self, delta_time: f64) {
        if self.window.is_key_down(Key::W) {
            self.camera.move_forward(&self.world, delta_time);
        }

        if self.window.is_key_down(Key::S) {
            self.camera.move_backward(&self.world, delta_time);
        }

        if self.window.is_key_down(Key::A) {
            self.camera.rotate_left(delta_time);
        }

        if self.window.is_key_down(Key::D) {
            self.camera.rotate_right(delta_time);
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