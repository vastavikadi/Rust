// Vectors
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    for num in &v {
        println!("{}", num);
    }
}

// HashMap
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    println!("{:?}", scores);
}

