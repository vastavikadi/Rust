// main.rs
mod utils;

fn main() {
    utils::say_hello();
}

// utils.rs
pub fn say_hello() {
    println!("Hello from utils!");
}
