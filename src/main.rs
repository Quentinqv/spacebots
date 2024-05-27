fn main() {
    println!("SpaceBots");
    let mut map = spacebots::game::map::Map::new(9, 9, 0);
    let robot = spacebots::game::robot::Robot::new(0, 0, map.clone());
    map.add_robot(robot);
    map.discover(4, 4);
    map.robots[0].move_down();

    map.display();
    map.display_with_vision();
}
