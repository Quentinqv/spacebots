fn main() {
    println!("SpaceBots");
    let mut map = spacebots::game::map::Map::new(10, 10, 0);
    let mut map2 = spacebots::game::map::Map::new(10, 10, 0);

    map.discover(5, 5);
    // wait for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));
    map2.discover(5, 5);

    println!("{}", map.tiles[5][5].last_time_visited);
    println!("{}", map2.tiles[5][5].last_time_visited);

    let merged_map = map.merge(&map2);
    merged_map.display();

    println!("{}", merged_map.tiles[5][5].last_time_visited);
}