use rand;
use rand::{random, Rng, SeedableRng};
use crate::entities::tiles::tile::{Tile, TileType};

use piston_window::*;
use rand::rngs::StdRng;

pub struct MapGame {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
}

impl MapGame {
    pub fn new(width: i32, height: i32, mut seed: u64) -> MapGame {
        if seed == 0 {
            println!("No seed provided, the map will be random");
            seed = random();
        }
        let mut rng = StdRng::seed_from_u64(seed);

        let mut tiles = vec![vec![Tile::new(TileType::Empty, (0, 0), true, false); height as usize]; width as usize];

        for x in 0..width {
            for y in 0..height {
                // Probability of each tile type is : Empty 40%, Rock 20%, Energy 35%, ScientificStation 5%
                let tile_type = match rng.gen_range(0..100) {
                    0..=39 => TileType::Empty,
                    40..=59 => TileType::Rock,
                    60..=94 => TileType::Energy,
                    _ => TileType::ScientificStation,
                };

                tiles[x as usize][y as usize].update_tile_type(tile_type);
            }
        }

        MapGame {
            tiles,
            width,
            height,
        }
    }

    pub fn display(&self) -> PistonWindow {
        let mut window: PistonWindow = WindowSettings::new("Piston Tutorial", [800, 800])
            .exit_on_esc(true)
            .build()
            .unwrap();

        while let Some(event) = window.next() {
            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g); // Clear window

                // Display the map in the window
                for x in 0..self.width {
                    for y in 0..self.height {
                        let tile = &self.tiles[x as usize][y as usize];
                        let color = [tile.color.0, tile.color.1, tile.color.2, 1.0];
                        let center = c.transform.trans((x * 80) as f64, (y * 80) as f64);
                        rectangle(color, [0.0, 0.0, 80.0, 80.0], center, g);
                    }
                }
            });
        }

        window
    }
}