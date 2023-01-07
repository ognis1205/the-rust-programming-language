use rand::Rng;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = Vec::new();
    let len = rng.gen_range(5..21);

    for _ in 0..len {
        numbers.push(rng.gen_range(-100..101));
    }
    println!("numbers: {:?}", numbers);
    println!("largest: {:?}", find(&numbers));

    let int_p = Point { x: 1, y: 2 };
    let flo_p = Point { x: 1.0, y: 2.0 };
    println!("integer: {:?}", int_p);
    println!("float: {:?}", flo_p);

    println!("integer x: {:?}", int_p.x());
    println!("float x: {:?}", flo_p.x());
}

fn find<T: PartialOrd + Copy>(xs: &[T]) -> T {
    let mut ret = xs[0];
    for &x in xs.iter() {
        if x > ret {
            ret = x;
        }
    }
    ret
}
