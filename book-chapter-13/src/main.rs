use std::{env::{self, Args}, process};

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

    let args = env::args();

    let config = parse_args(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("{}{}", &config.file_name, &config.query);

    // let args: Vec<String> = args.collect();
}

struct Config {
    file_name: String,
    query: String,
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let file_name = match args.next() {
        Some(arg) => arg,
        None => return Err("Argument not exists"),
    };

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Argument not exists"),
    };

    Ok(Config {
        file_name: file_name,
        query: query,
    })
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