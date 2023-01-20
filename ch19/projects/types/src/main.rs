fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
    f();
}

type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;
