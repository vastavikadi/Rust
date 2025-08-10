fn main() {
    greet("Rust");

    let sum = add(5, 7);
    println!("Sum = {}", sum);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // return without `;`
}
