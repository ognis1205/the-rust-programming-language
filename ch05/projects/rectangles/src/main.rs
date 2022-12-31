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
