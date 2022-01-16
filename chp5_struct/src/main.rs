fn main() {
    let user = User {
        email: String::from("gdhu@126.com"),
        username: String::from("gdhu"),
        activate: true,
        sign_in_count: 20,
    };
    println!("{}", user.username);

    let user = build_user("gdhu@126.com".to_string(), "gdhu".to_string());
    println!("{}", user.email);

    let user2 = User {
        email: String::from("gdhu1@126.com"),
        ..user
    };
    println!("{}", user2.email);

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        activate: true,
        sign_in_count: 1,
    }
}

struct User {
    activate: bool,
    sign_in_count: u32,
    username: String,
    email: String
}

struct Color(i32, i32, i32);

struct AlwaysEqual;