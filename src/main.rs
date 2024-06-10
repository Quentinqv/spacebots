use piston_window::*;
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

const TILE_SIZE: f64 = 32.0;
const INFO_WIDTH: f64 = 330.0;

fn create_window(width: u32, height: u32) -> PistonWindow {
    let window: PistonWindow = WindowSettings::new(
        "Map Display",
        [
            width * TILE_SIZE as u32 + INFO_WIDTH as u32,
            height * TILE_SIZE as u32,
        ],
    )
        .exit_on_esc(true)
        .build()
        .unwrap();
    window
}

fn draw_map(window: &mut PistonWindow, event: &Event, map: &Map, robots: &[Robot], glyphs: &mut Glyphs) {
    window.draw_2d(event, |c, g, device| {
        clear([1.0; 4], g);

        for (x, row) in map.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let color = if tile.is_discovered {
                    match tile.tile_type {
                        game::tile::TileType::Empty => [1.0, 1.0, 1.0, 1.0],
                        game::tile::TileType::Rock => [0.5, 0.5, 0.5, 1.0],
                        game::tile::TileType::Energy => [1.0, 1.0, 0.0, 1.0],
                        game::tile::TileType::ScientificStation => [0.0, 0.0, 1.0, 1.0],
                    }
                } else {
                    [0.0, 0.0, 0.0, 1.0]
                };

                rectangle(
                    color,
                    [
                        x as f64 * TILE_SIZE,
                        y as f64 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }
        }

        for robot in robots {
            let robot_color = [1.0, 0.0, 0.0, 1.0];
            ellipse(
                robot_color,
                [
                    robot.x as f64 * TILE_SIZE + TILE_SIZE * 0.25,
                    robot.y as f64 * TILE_SIZE + TILE_SIZE * 0.25,
                    TILE_SIZE * 0.5,
                    TILE_SIZE * 0.5,
                ],
                c.transform,
                g,
            );
        }

        let info_x = map.width as f64 * TILE_SIZE;
        rectangle(
            [0.8, 0.8, 0.8, 1.0],
            [info_x, 0.0, INFO_WIDTH, map.height as f64 * TILE_SIZE],
            c.transform,
            g,
        );

        let mut text_y = 20.0;
        let font_size = 20;
        for robot in robots {
            let energy_text = format!(
                "Robot {}: Energy collected = {}",
                robot.id, robot.energy_collected
            );
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], font_size)
                .draw(
                    &energy_text,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(info_x + 10.0, text_y),
                    g,
                )
                .unwrap();
            text_y += 30.0;
        }

        glyphs.factory.encoder.flush(device);
    });
}

fn launch_game(map: Arc<Mutex<Map>>, robots: Arc<Mutex<Vec<Robot>>>, steps: usize) {
    let map_clone = Arc::clone(&map);
    let map = map.lock().unwrap();
    let mut window = create_window(map.width as u32, map.height as u32);
    drop(map);

    let font_data = include_bytes!("./assets/FiraSans-Regular.ttf");
    let mut glyphs = Glyphs::from_bytes(font_data, window.create_texture_context(), TextureSettings::new()).unwrap();

    let mut handles = vec![];
    for i in 0..robots.lock().unwrap().len() {
        let _map_clone = Arc::clone(&map_clone);
        let robots_clone = Arc::clone(&robots);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..steps {
                {
                    let mut robots = robots_clone.lock().unwrap();
                    let robot = &mut robots[i];
                    let direction = rng.gen_range(0..4);
                    match direction {
                        0 => robot.move_up(),
                        1 => robot.move_down(),
                        2 => robot.move_left(),
                        3 => robot.move_right(),
                        _ => (),
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }

    while let Some(event) = window.next() {
        {
            let map = map_clone.lock().unwrap();
            let robots = robots.lock().unwrap();
            draw_map(&mut window, &event, &map, &robots, &mut glyphs);
        }
        thread::sleep(Duration::from_millis(16));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let seed = 42;

    let map = Arc::new(Mutex::new(Map::new(width, height, seed)));
    let robots = Arc::new(Mutex::new(vec![
        Robot::new(1, 0, 0, Arc::clone(&map)),
        Robot::new(2, 6, 6, Arc::clone(&map)),
    ]));

    {
        let mut map = map.lock().unwrap();
        for robot in &*robots.lock().unwrap() {
            map.discover(robot.x as i32, robot.y as i32);
        }
    }

    launch_game(Arc::clone(&map), Arc::clone(&robots), 100);
}
