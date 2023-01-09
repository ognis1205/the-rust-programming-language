struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: std::cmp::PartialOrd> Rectangle<T> {
    fn can_hold(&self, other: &Rectangle<T>) -> bool {
        self.width > other.width && self.height > other.height
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
}
