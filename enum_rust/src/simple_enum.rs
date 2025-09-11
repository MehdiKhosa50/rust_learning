pub fn run() {
    println!("Running simple_enum module...");
    getting_direction();
}

fn getting_direction() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    match dir {
        Direction::North => println!("Heading North!"),
        Direction::East => println!("Heading East!"),
        Direction::South => println!("Heading South!"),
        Direction::West => println!("Heading West!"),
    }
}
