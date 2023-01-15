use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);
    {
        match m.lock() {
            Ok(mut num) => {
                *num = 6;
            }
            Err(err) => {
                eprintln!("error occured: {:?}", err);
            }
        }
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap_or_else(|err| {
                panic!("failed to lock");
            });
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Err(err) => {
                eprintln!("error occured: {:?}", err);
            }
            _ => (),
        }
    }
    println!("Result: {}", *counter.lock().unwrap());
}
