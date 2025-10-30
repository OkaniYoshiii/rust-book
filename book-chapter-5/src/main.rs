fn main() {
    println!("Hello, world!");

    let user = User {
        active: true,
        email: String::from("user@mail.fr"),
        username: String::from("OkaniYoshiii"),
        login_attemps: 0,
    };

    let admin = build_user(String::from("admin@mail.fr"), String::from("Admin"));
    let _admin = User::admin(String::from("admin"), String::from("admin@mail.fr"));

    let _other_user = User {
        email: String::from("other-user@mail.fr"),
        username: String::from("Other user"),
        ..user
    };

    dbg!(admin);
    println!("{}{}{}{}", user.active, user.email, user.username, user.login_attemps);
    println!("{user:?}");
    println!("{}", user.can_log_in());
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    login_attemps: u8,
}

impl User {
    fn can_log_in(&self) -> bool {
        self.login_attemps < 3
    }

    fn admin(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: false,
            login_attemps: 0,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        login_attemps: 0,
    }
}