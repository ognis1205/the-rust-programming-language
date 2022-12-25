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
}
