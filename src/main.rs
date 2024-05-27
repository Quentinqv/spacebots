use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use crate::game::map::Map;
use crate::game::robot::Robot;

mod game {
    pub mod map;
    pub mod robot;
    pub mod tile;
}

fn display_map(map: &Map, robots: &[Robot]) {
    let mut display = vec![vec![' '; map.width as usize]; map.height as usize];

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.is_discovered {
                let symbol = match tile.tile_type {
                    game::tile::TileType::Empty => ' ',
                    game::tile::TileType::Rock => '#',
                    game::tile::TileType::Energy => 'E',
                    game::tile::TileType::ScientificStation => 'S',
                };
                display[y][x] = symbol;
            } else {
                display[y][x] = '-';
            }
        }
    }

    for robot in robots {
        display[robot.y][robot.x] = 'R';
    }

    for row in display {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
    println!();
}

fn launch_game(map: Arc<Mutex<Map>>, robots: &mut [Robot], steps: usize) {
    for _ in 0..steps {
        let mut rng = rand::thread_rng();

        for robot in robots.iter_mut() {
            let direction = rng.gen_range(0..4);

            match direction {
                0 => robot.move_up(),
                1 => robot.move_down(),
                2 => robot.move_left(),
                3 => robot.move_right(),
                _ => (),
            }
        }

        {
            let map = map.lock().unwrap();
            display_map(&map, robots);
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let seed = 42; // Fixed seed for reproducibility

    let map = Arc::new(Mutex::new(Map::new(width, height, seed)));
    let mut robots = vec![
        Robot::new(1, 2, 2, Arc::clone(&map)),
        Robot::new(2, 6, 6, Arc::clone(&map)),
    ];

    {
        let mut map = map.lock().unwrap();
        // Discover map where robots are
        for robot in &robots {
            map.discover(robot.x as i32, robot.y as i32);
        }
        println!("Initial Map:");
        display_map(&map, &robots);
    }

    // Launch the game for 20 steps
    launch_game(Arc::clone(&map), &mut robots, 20);
}
