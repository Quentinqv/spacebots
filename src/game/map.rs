use rand;
use rand::{random, Rng, SeedableRng};

use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use crate::game::robot::Robot;
use crate::game::tile::{Tile, TileType};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
    pub robots: Vec<Robot>,
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

        Map {
            tiles,
            width,
            height,
            robots: vec![],
        }
    }

    pub fn discover(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        self.tiles[x as usize][y as usize].visit();
    }

    pub fn merge(&self, other: &Map) -> Map {
        let mut new_map = self.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                if other.tiles[x as usize][y as usize].last_time_visited > self.tiles[x as usize][y as usize].last_time_visited {
                    new_map.tiles[x as usize][y as usize] = other.tiles[x as usize][y as usize];
                }
            }
        }
        new_map
    }

    pub fn display(&self) {
        // Clear the screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut robot = false;
                for r in &self.robots {
                    if r.x == x as usize && r.y == y as usize {
                        robot = true;
                    }
                }
                if robot {
                    print!("X");
                } else {
                    match self.tiles[x as usize][y as usize].tile_type {
                        TileType::Empty => print!(" "),
                        TileType::Rock => print!("#"),
                        TileType::Energy => print!("*"),
                        TileType::ScientificStation => print!("S"),
                    }
                }
            }
            println!();
        }
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    // Display the map with the robot's vision
    pub fn display_with_vision(&self) {
        // Clear the screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut robot = false;
                for r in &self.robots {
                    if r.x == x as usize && r.y == y as usize {
                        robot = true;
                    }
                }
                if robot {
                    print!("X");
                } else {
                    // if tile.is_discovered, print the tile
                    // else print a space
                    // println!("{}", self.tiles[x as usize][y as usize].is_discovered.to_string());
                    if self.tiles[x as usize][y as usize].is_discovered {
                        match self.tiles[x as usize][y as usize].tile_type {
                            TileType::Empty => print!(" "),
                            TileType::Rock => print!("#"),
                            TileType::Energy => print!("*"),
                            TileType::ScientificStation => print!("S"),
                        }
                    } else {
                        print!("-");
                    }
                }
            }
            println!();
        }
    }
}