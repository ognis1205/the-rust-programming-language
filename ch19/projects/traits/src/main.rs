use std::ops::Add;
use traits::*;

fn main() {
    let counter = Counter::new();
    for i in counter {
        println!("counter: {}", i);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let milli = Millimeters(1234);
    let meter = Meters(2);
    println!("meter: {:?}", milli + meter);

    let person = Human;
    Wizard::fly(&person);
    Pilot::fly(&person);
    person.fly();
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

struct Human;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("dog")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("animal")
    }
}
