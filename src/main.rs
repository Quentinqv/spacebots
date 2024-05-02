use spacebots::networking;

fn main() {
    println!("SpaceBots");
    let _map = spacebots::game::map::Map::new(10, 10, 0);

    println!("Starting server...");
    networking::server::main();

    println!("Sending map...");
}