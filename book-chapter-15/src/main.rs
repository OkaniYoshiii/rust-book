use std::ops::Deref;

fn main() {
    println!("Hello, world!");

    let val = Box::new(5);
    println!("{val}");

    let user = User {
        user: Some(&User {
            user: None,
        })
    };

    println!("{:?}", user.user);

    let number = Byte(15);
    let number = number.add(&Byte(20));
    println!("{number}");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[derive(Debug)]
struct User<'a> {
    user: Option<&'a User<'a>>,
}

struct Byte(u8);

impl Byte {
    fn add(&self, byte: &Byte) -> u8 {
        &self.0 + byte.0
    }
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
