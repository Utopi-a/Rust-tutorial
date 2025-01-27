struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("some_username123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another_username567"),
        ..user1
    };

    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
