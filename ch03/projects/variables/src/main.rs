use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "     ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    loop {
        let guess: u32 = match "42".trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The value of guess is: {}", guess);
        break;
    }

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {} is {}", index, element);
}
