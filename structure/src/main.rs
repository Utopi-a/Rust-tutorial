struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
