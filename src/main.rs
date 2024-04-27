mod entities {
    pub mod map {
        pub mod map_game;
    }
    pub mod tiles {
        pub mod tile;
    }
}

use entities::map::map_game::MapGame;

fn main() {
    println!("SpaceBots");

    let map = MapGame::new(10, 10, 0);
    let window = map.display();
}