struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("User1: {}", user1.email);
    println!("User1: {}", user1.username);

    // let user2 = User {
    //     email: String::from("example@user.com"),
    //     username: String::from("userexample"),
    //     ..user1
    // };

    

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("User2: {}", user2.email);
    println!("User2: {}", user2.username);
}