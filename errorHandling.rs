use std::fs::File;
use std::io::{ self, Read };

fn main() -> io::Result<()> {
    // Result<T, E> — used for recoverable errors.
    // ? — propagates error upward.

    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
