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

    let first = "bonjour";
    let longest_str : &str;

    {
        let second = String::from("second");
        longest_str = longest(first, &second);
        println!("{longest_str}");
    }

    // Ne marche pas car "second" sort du scope avant cette ligne
    // longest_str peut être une référence vers "second", donc cela ne marche pas
    // println!("{longest_str}");

    println!("{}{}", point.x(), point.y);

    let mut square = Square {
        point: &Point {
            x: 1,
            y: 1,
        }
    };
    
    {
        let point = Point { x: 1, y: 1 };
        square.point = &point;

        println!("{}", square.point.x);
        println!("{}", square.width());
    }

    // Ne marche pas non plus, cat point dans le scope le plus
    // petit ne vit pas assez longtemps
    // println!("{}", square.point.x);

    let mut circle = Circle {
        point: Point{
            x: 0,
            y: 0,
        }
    };

    {
        let point = Point { x: 0, y: 0 };
        circle.point = point;

        println!("{}", circle.point.x);
        // Ne marche car la valeur est déplacée ("moved") dans la struct
        // println!("{}", point.x);
    }

    // Marche car la valeur est déplacée ("moved") dans la struct
    println!("{}", circle.point.x);
    const VALUE: &str = "bonjour";
    println!("{VALUE}");
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() { first } else { second }
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

struct Square<'a> {
    point: &'a Point<i32>,
}

impl<'a> Square<'_> {
    fn width(&self) -> i32 {
        150
    }
}

struct Circle {
    point: Point<i32>,
}