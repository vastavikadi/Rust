fn main() {
    // String — owned, mutable.
    // &str — borrowed, immutable slice.
    
    let s1 = String::from("Hello"); // heap-allocated, growable
    let s2 = "World"; // string slice, fixed
    let s3 = format!("{}, {}", s1, s2);
    println!("{}", s3);
}
