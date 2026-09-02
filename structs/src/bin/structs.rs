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

fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("lala"),
        email: String::from("lala@lala.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("lala"),
        email: String::from("lala@lala.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("lala2@lala.com");

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("lele@lala.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("lele@lala.com"),
        ..user1
    };

    

    // println!("user1 {:#?}", user1); // não implementa Debug
}
