use std::collections::{HashMap};

enum Row {
    Id(u32),
    Name(String),
}

fn main() {
    println!("Hello, world!");

    let _ages = Vec::<u32>::new();
    let _ages: Vec<u32> = Vec::new();
    let mut ages = vec![8];

    ages.push(10);

    let _first = &ages[0];

    let first = ages.get(0);
    match first {
        Some(first) => println!("{first}"),
        None => return,
    };

    let Some(_first) = ages.get(0) else {
        return;
    };

    for age in &ages {
        println!("{age}");
    }

    let row = vec![
        Row::Id(30),
        Row::Name(String::from("User")),
    ];

    for cell in &row {
        match cell {
            Row::Id(val) => println!("{val}"),
            Row::Name(val) => println!("{val}"),
        }
    }

    println!("{}", row.len());

    let data = "Bonjour";
    let _data = data.to_string();
    let mut data = "une jolie école !".to_string();
    let name = String::from("User");

    data.push('b');
    data.push_str(&name);
    let _sentence = format!("bonjour {}", data);

    let sentence = data + &name;

    let start = &sentence[0..3];

    for char in sentence.chars() {
        println!("{char}");
    }

    println!("{sentence}{name}{start}");

    let _form_data = HashMap::<&str, &str>::new();
    let _form_data: HashMap<&str, &str> = HashMap::new();
    let mut form_data = HashMap::new();

    form_data.insert("password", "val");

    let _password = form_data.get("password").expect("No password defined");
    let _password = form_data.get("password").copied().unwrap_or("");

    for (name, value) in &form_data {
        println!("{name}, {value}");
    }

    form_data.insert("password", "new_password");
    form_data.entry("password").or_insert("default_password");
    let _entry = form_data.entry("password");
    let _entry = form_data.entry("password").or_insert("default_password");
}
