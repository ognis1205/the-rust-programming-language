pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

impl<T: std::cmp::PartialOrd> Rectangle<T> {
    pub fn can_hold(&self, other: &Rectangle<T>) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    let mut ret = String::from("Hello!");
    ret.push_str(name);
    ret
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value: value }
    }
}
