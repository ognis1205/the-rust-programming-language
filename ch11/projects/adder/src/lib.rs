pub struct Rectangle<T> {
    width: T,
    height: T,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        let x = 5;
        assert_eq!(7, add_two(x));
    }

    #[test]
    fn it_greets() {
        let name = String::from("ognis1205");
        //        let name = String::from("Carol");
        let result = greeting(name.as_str());
        assert!(
            result.contains("ognis1205"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101")]
    fn greater_than_100() {
        let guess = Guess::new(101);
    }
}
