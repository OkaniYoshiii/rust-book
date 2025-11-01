use core::panic;
use std::{collections::HashMap, fs::File, io::{Error, ErrorKind, Read}};

fn main() {
    // panic!("Oh oh !");

    let filepath = "error.log";
    let file = File::open(filepath);
    let _file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(filepath) {
                Ok(file) => file,
                Err(err) => panic!("Problem creating the file : {err:?}"),
            },
            _ => panic!("Problem opening the fie : {err:?}"),
        },
    };

    let _file = File::open("error.log").unwrap();
    let _file = File::open(filepath).expect("Cannot open file");
    let _file_result = read_posts();
    let _file_result = read_file("error.log");
    let _user = get_user("user@mail.fr");
}

fn read_posts() -> Result<File, Error> {
    let posts_result = File::open("posts.json");

    let posts_file = match posts_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    Ok(posts_file)
}

fn read_file(filepath: &str) -> Result<String, Error> {
    let mut buf = String::new();

    File::open(filepath)?.read_to_string(&mut buf)?;

    Ok(buf)
}

fn get_user(user: &str) -> Option<&str> {
    let users = HashMap::from([
        ("user@mail.fr", "Thomas"),
        ("admin@mail.fr", "Julie"),
    ]);

    Some(users.get(user).copied()?)
}