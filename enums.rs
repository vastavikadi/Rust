enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        _ => println!("Sideways"),
    }
}
