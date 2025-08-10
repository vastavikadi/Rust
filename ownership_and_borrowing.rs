fn main() {
    // Ownership is the heart of Rust’s memory safety.

    // Move — default for non-copy types.
    // Clone — explicitly duplicate.
    // Borrow (&) — pass reference without taking ownership.
    // Mutable borrow (&mut) — modify through reference.
    
    let s1 = String::from("hello");
    let s2 = s1; // moves ownership, s1 is invalid now

    // println!("{}", s1); // ERROR

    let s3 = s2.clone(); // deep copy
    println!("s2 = {}, s3 = {}", s2, s3);

    let len = calculate_length(&s3); // borrow with &
    println!("Length = {}", len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
