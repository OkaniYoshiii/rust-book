#[allow(dead_code)]

enum IpAddr {
    V4(String),
    V6(String),
}

enum Color {
    Yellow,
    Red,
}

impl Color {
    fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Red => (255, 0, 0),
            _ => (0, 0, 0),
        }
    }

    fn print(&self) -> () {
        let (r, g, b) = self.rgb();
        println!("{r}, {g}, {b}")
    }
}

enum Fruit {
    Tomato(Color),
    Banana(Color),
}

fn main() {
    println!("Hello, world!");

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V6(String::from("::1"));

    let _ip_addr = match home {
        IpAddr::V4(val) => val,
        IpAddr::V6(val) => val,
    };

    let _some_number = Option::Some(5);
    let _some_number = Some(5);

    let _number = match _some_number {
        Some(val) => val,
        None => 0,
    };

    let dice_roll = 5;

    let action = match dice_roll {
        1 => "Cast spell",
        6 => "Kill enemy in one shot",
        _ => "Do nothing",
    };

    match action {
        "Cast spell" => println!("You've casted a spell !"),
        "Kill enemy in one shot" => println!("You've killed the enemy in one shot"),
        other => println!("{other}"),
    }

    let database_config = Some("config");
    if let Some(config) = database_config {
        println!("{config}");
    }

    let fruit = Fruit::Tomato(Color::Red); 
    if let Fruit::Tomato(color) = fruit {
        color.print();
    } else {
        println!("");
    }

    let banana = Fruit::Banana(Color::Yellow);
    let Fruit::Banana(_color) = banana else {
        return ();
    };

    println!("{action}");
}

