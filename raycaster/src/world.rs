use crate::color::*;

pub struct World {
    width: Option<i64>,
    height: Option<i64>,
    map: Option<Vec<u8>>,
}

impl World {
    pub fn new() -> Self {

        Self {
            width: None,
            height: None,
            map: None,
        }
    }

    pub fn init(&mut self, width: i64, height: i64, map: Vec<u8>){
        self.width = Some(width);
        self.height = Some(height);
        self.map = Some(map);
    } 

    pub fn is_initialized(&self) -> bool {
        self.width.is_some() && self.height.is_some() && self.map.is_some()
    }

    pub fn is_collision(&self, xpos: i64, ypos: i64) -> bool {
        // For now we will treat the uninitialized map as out of bounds
        if !self.is_initialized() {
            return true;
        }

        let width = self.width.unwrap();
        let height = self.height.unwrap();

        // Treat out of bounds as being a collision
        if xpos < 0 || xpos >= width || ypos < 0 || ypos >= height {
            return true;
        }

        let index = ypos * width + xpos;
        self.map.as_ref().unwrap()[index as usize] > 0
    }

    // TODO: This needs to have bounds checking
    pub fn get_cell(&self, xpos: i64, ypos: i64) -> u8 {
        let width = self.width.unwrap();
        let index = ypos * width + xpos;
        self.map.as_ref().unwrap()[index as usize]
    }
}