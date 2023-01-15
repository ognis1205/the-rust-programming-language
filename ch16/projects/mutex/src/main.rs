use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        match m.lock() {
            Ok(mut num) => {
                *num = 6;
            }
            Err(err) => {
                eprintln!("error occured: {}", err);
            }
        }
    }
    println!("m = {:?}", m);
}
