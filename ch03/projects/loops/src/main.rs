fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut l = 0;
    while l < a.len() {
        println!("The value is: {}", a[l]);
        l += 1;
    }

    let a = ['a', 'b', 'c', 'd', 'e'];
    for e in a {
        println!("the value is: {}", e);
    }

    for e in (1..4).rev() {
        println!("the value is: {}", e);
    }
}
