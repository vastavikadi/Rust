fn main() {
    let x = 5; // let — immutable by default
    let mut y = 10; // let mut — mutable
    println!("x = {}, y = {}", x, y);

    y = 20;
    println!("Updated y = {}", y);

    // Constants use const and need type annotations
    const PI: f64 = 3.1415;//here f64 is a type and defining types like we do in ts
    println!("PI = {}",PI) //"{}" - a string literal that is must for printing the results
}
