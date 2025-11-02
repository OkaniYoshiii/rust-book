fn main() {
    let users: Vec<String> = Vec::from([
        String::from("OkaniYoshiii"),
        String::from("Itsuki"),
    ]);


    let prefix = "user";
    let prefix_user = |str: &String| format!("{prefix} : {str}");
    for user in users.iter().map(prefix_user) {
        println!("{user}");
    }

    let mut user = User {
        name: String::from("OkaniYoshiii"),
        password: String::from("password"),
    };

    user.for_each(|field| { field.push_str(" field"); });

    println!("{user:?}");
}

#[derive(Debug)]
struct User {
    name: String,
    password: String,
}

impl User {
    fn for_each<F: Fn(&mut String)>(&mut self, callback: F) {
        let values = Vec::from([
            &mut self.name,
            &mut self.password,
        ]);

        for value in values {
            callback(value);
        }
    }
}