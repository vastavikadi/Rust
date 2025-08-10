fn main() {
    // Rust is statically typed, but can infer most types.
    //here we are defining the types of variables as we do in ts
    let a: i32 = 10; // integer
    let b: f64 = 3.14; // float
    let c: bool = true; //boolean
    let d: char = 'R'; //characters
    let e: &str = "Rust"; // string slice

    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    // Numeric Types
    // Integers: i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, etc.
    // Floats: f32, f64
}
