fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s1 = takes_and_gives_back(s1);
    println!("s1 = {}", s1);

    let mut s1 = s1;
    s1.push_str(" world!");
    println!("s1 = {}", s1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(mut some_string: String) -> String {
    some_string.push_str(" world!");
    some_string
}
