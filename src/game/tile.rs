use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: (i32, i32),
    pub traversable: bool,
    pub is_discovered: bool,
    pub color: (f32, f32, f32),
    pub last_time_visited: u128,
}

impl Tile {
    pub fn new(tile_type: TileType, position: (i32, i32), traversable: bool, is_discovered: bool) -> Tile {
        Tile {
            tile_type,
            position,
            traversable,
            is_discovered,
            color: (0.0, 0.0, 0.0),
            last_time_visited: 0,
        }
    }

    pub fn update_traversable(&mut self) {
        self.traversable = match self.tile_type {
            TileType::Empty => true,
            TileType::Rock => false,
            TileType::Energy => true,
            TileType::ScientificStation => true,
        };
    }

    pub fn update_color(&mut self) {
        self.color = match self.tile_type {
            TileType::Empty => (0.0, 0.0, 0.0), // black
            TileType::Rock => (0.5, 0.3, 0.1), // brown
            TileType::Energy => (0.0, 1.0, 0.0), // green
            TileType::ScientificStation => (0.5, 0.1, 0.5), // purple
        };
    }

    pub fn update_tile_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
        self.update_traversable();
        self.update_color();
    }

    pub fn visit(&mut self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        self.last_time_visited = timestamp;
        self.is_discovered = true;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Rock,
    Energy,
    ScientificStation,
}
