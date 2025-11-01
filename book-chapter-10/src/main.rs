use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let list = vec![0, 10, 20];
    let _first = first(&list);

    let mut point = Point {
        x: 10,
        y: 10,
    };

    point.inverse_x();

    let rectangle = Rectangle {
        width: 32,
    };

    rectangle.hello();
    hello(&rectangle);
    same_hello(&rectangle, &rectangle);

    println!("{}{}", point.x(), point.y);
}

#[allow(dead_code)]
fn multiple<T: Stringable + Display>(_item: T) {
    todo!()
}

#[allow(dead_code)]
fn other_multiple<T>(_item: T) -> impl Stringable + Display
where
    T: Stringable + Display,
{
    return _item;
}

fn same_hello<T:Stringable>(first: &T, second: &T) -> String {
    let mut first_str = first.hello();
    let second_str = second.hello();

    first_str.push_str(&second_str);

    first_str
}

fn hello(val: &impl Stringable) -> String {
    val.string()
}

fn first<T: std::marker::Copy>(list: &[T]) -> Option<T> {
    list.get(0).copied()
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn inverse_x(&mut self) {
        self.x *= -1;
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Rectangle {
    width: i32,
}

impl Stringable for Rectangle {
    fn hello(&self) -> String {
        format!("Hello {}", self.string())
    }
    fn string(&self) -> String {
        format!("{}", self.width)
    }
}

trait Stringable {
    fn hello(&self) -> String;
    fn string(&self) -> String {
        String::new()
    }
}