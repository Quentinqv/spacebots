use serde::{Deserialize, Serialize};
use crate::game::map::Map;
use crate::game::tile::TileType;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Robot {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub map: Arc<Mutex<Map>>,
    pub local_map: Map,
    pub energy_collected: u32,
}

impl Robot {
    pub fn new(id: u32, x: usize, y: usize, map: Arc<Mutex<Map>>) -> Robot {
        let local_map = map.lock().unwrap().clone();  // Each robot gets its own copy of the map
        Robot {
            id,
            x,
            y,
            map,
            local_map,
            energy_collected: 0,
        }
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

        // Étape 1: Vérifiez si les nouvelles coordonnées sont dans les limites et traversables
        let traversable = {
            let map = self.map.lock().unwrap();
            self.is_within_bounds(new_x, new_y) && self.is_traversable(new_x, new_y, &map)
        };

        if traversable {
            // Mise à jour des coordonnées du robot
            self.x = new_x;
            self.y = new_y;

            // Étape 2: Collecter les ressources et découvrir la case avec des emprunts mutables
            self.collect_and_discover(new_x, new_y);
        } else {
            // Marquez la case comme découverte même si elle n'est pas traversable
            self.mark_discovered(new_x, new_y);
            // Revenir à la position précédente si non traversable
            self.x = old_x;
            self.y = old_y;
        }

        // Étape 3: Fusionnez les cartes si le robot est sur une station scientifique
        if let TileType::ScientificStation = self.local_map.tiles[self.x][self.y].tile_type {
            self.merge_maps();
        }
    }

    fn collect_and_discover(&mut self, x: usize, y: usize) {
        let mut shared_map = self.map.lock().unwrap();
        let tile_type = shared_map.tiles[x][y].tile_type.clone();
        match tile_type {
            TileType::Energy => {
                self.energy_collected += 1;
                shared_map.tiles[x][y].tile_type = TileType::Empty; // La case est maintenant vide après avoir collecté l'énergie
                shared_map.tiles[x][y].update_traversable();
                shared_map.tiles[x][y].update_color();
            }
            TileType::Rock => {}
            TileType::ScientificStation => {}
            _ => {}
        }
        shared_map.discover(x as i32, y as i32);
        self.local_map.discover(x as i32, y as i32);
    }

    fn mark_discovered(&mut self, x: usize, y: usize) {
        let mut shared_map = self.map.lock().unwrap();
        shared_map.discover(x as i32, y as i32);
        self.local_map.discover(x as i32, y as i32);
    }

    fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.local_map.width as usize && y < self.local_map.height as usize
    }

    fn is_traversable(&self, x: usize, y: usize, map: &Map) -> bool {
        if x >= map.width as usize || y >= map.height as usize {
            return false;
        }
        let tile = &map.tiles[x][y];
        tile.traversable
    }

    fn merge_maps(&mut self) {
        let shared_map = self.map.lock().unwrap();

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
