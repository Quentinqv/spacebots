use serde::{Deserialize, Serialize};
use crate::game::map::Map;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Robot {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub map: Arc<Mutex<Map>>,  // Shared map for initial seeding
    pub local_map: Map,        // Local copy of the map
}

impl Robot {
    pub fn new(id: u32, x: usize, y: usize, map: Arc<Mutex<Map>>) -> Robot {
        let local_map = map.lock().unwrap().clone();  // Each robot gets its own copy of the map
        Robot { id, x, y, map, local_map }
    }

    pub fn move_up(&mut self) {
        self.move_to(self.x, self.y.saturating_sub(1));
    }

    pub fn move_down(&mut self) {
        self.move_to(self.x, self.y + 1);
    }

    pub fn move_left(&mut self) {
        self.move_to(self.x.saturating_sub(1), self.y);
    }

    pub fn move_right(&mut self) {
        self.move_to(self.x + 1, self.y);
    }

    fn move_to(&mut self, new_x: usize, new_y: usize) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = new_x;
        self.y = new_y;

        let mut shared_map = self.map.lock().unwrap();
        if self.is_traversable(new_x, new_y) {
            shared_map.discover(new_x as i32, new_y as i32);
            self.local_map.discover(new_x as i32, new_y as i32);
        } else {
            // Mark the non-traversable tile as discovered
            shared_map.discover(new_x as i32, new_y as i32);
            self.local_map.discover(new_x as i32, new_y as i32);
            // Revert to the previous position
            self.x = old_x;
            self.y = old_y;
        }

        // Check if the current tile is a ScientificStation
        let current_tile = self.local_map.tiles[self.x][self.y].tile_type.clone();
        drop(shared_map); // Release the lock before calling merge_maps

        if let crate::game::tile::TileType::ScientificStation = current_tile {
            self.merge_maps();
        }
    }

    fn is_traversable(&self, x: usize, y: usize) -> bool {
        if x >= self.local_map.width as usize || y >= self.local_map.height as usize {
            return false;
        }
        let tile = &self.local_map.tiles[x][y];
        tile.traversable
    }

    fn merge_maps(&mut self) {
        let shared_map = self.map.lock().unwrap(); // Reacquire the lock

        for x in 0..self.local_map.width {
            for y in 0..self.local_map.height {
                let shared_tile = &shared_map.tiles[x as usize][y as usize];
                let local_tile = &mut self.local_map.tiles[x as usize][y as usize];

                if shared_tile.last_time_visited > local_tile.last_time_visited {
                    *local_tile = shared_tile.clone();
                }
            }
        }
    }
}
