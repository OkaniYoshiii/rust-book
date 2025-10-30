fn main() {
    let mut person  = String::from("Thomas");

    print_person(&mut person);

    println!("{}", person);

    let person = String::from("Antony");

    let slice = &person[0..2];

    println!("{}", slice)
}

fn print_person(person: &mut String) {
    *person = String::from("bonjour");
    println!("{}", person);
}
