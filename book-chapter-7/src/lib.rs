pub mod restaurant;

fn testing() {
    println!("test");
}

pub mod serving {
    use super::restaurant;

    pub fn take_order() {
        restaurant::say_hello();
        super::testing();

        println!("Can I take your order please ?");
    }
}