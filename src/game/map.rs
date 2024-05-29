use rand::{random, Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use crate::game::tile::{Tile, TileType};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32, mut seed: u64) -> Map {
        if seed == 0 {
            println!("No seed provided, the map will be random");
            seed = random();
        }
        let mut rng = StdRng::seed_from_u64(seed);

        let mut tiles = vec![vec![Tile::new(TileType::Empty, (0, 0), true, false); height as usize]; width as usize];

        for x in 0..width {
            for y in 0..height {
                let tile_type = match rng.gen_range(0..100) {
                    0..=39 => TileType::Empty,
                    40..=59 => TileType::Rock,
                    60..=94 => TileType::Energy,
                    _ => TileType::ScientificStation,
                };
                tiles[x as usize][y as usize].update_tile_type(tile_type);
            }
        }

        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn discover(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        self.tiles[x as usize][y as usize].visit();
    }
}
