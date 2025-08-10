struct User {
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("Alice"),
        active: true,
    };

    println!("{} is active: {}", user1.username, user1.active);
}
