fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The rectanle is {:?}", rect);
    println!("The area of rectanle is {}", rect.area());

    let p1 = Point { x: 1.5, y: -1.5 };
    let p2 = Point { x: 10.0, y: 15.5 };

    println!("The points are {:?}, {:?}", p1, p2);
    println!("The distance of pooints is {}", p1.distance(&p2));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("The rectanle is {:?}", rect1);
    println!("The rectanle is {:?}", rect2);
    println!("The rectanle is {:?}", rect3);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_2 = f64::powi(other.x - self.x, 2);
        let y_2 = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_2 + y_2)
    }
}
