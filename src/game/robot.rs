use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::game::map::Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub map: Map,
}

impl Robot {
    pub fn new(x: usize, y: usize, map: Map) -> Robot {
        Robot {
            x,
            y,
            map,
        }
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
        self.map.discover(self.x as i32, self.y as i32);
    }

    pub fn move_down(&mut self) {
        self.y += 1;
        self.map.discover(self.x as i32, self.y as i32);
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
        self.map.discover(self.x as i32, self.y as i32);
    }

    pub fn move_right(&mut self) {
        self.x += 1;
        self.map.discover(self.x as i32, self.y as i32);
    }
}