struct User {
    username:  String,
    email: String,
    sign_in_count:  u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut u = User {
        email: String::from("name@email.com"),
        username: String::from("Woop"),
        active: true,
        sign_in_count: 100_001,
    };

    u.active = false;

    let u2 = build_user(String::from("Don"), "don@don.com".to_string());

    let u3 =  User {
        email: String::from("hello@email.com"),
        username: String::from("Joe Schmo"),
        ..u2
    };
}
