fn main() {
    another_function(5);
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(1);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
