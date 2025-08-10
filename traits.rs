trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let p = Person { name: String::from("Bob") };
    p.greet();
}
