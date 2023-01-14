use std::process;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sent: {}", val);
        tx.send(val).unwrap_or_else(|err| {
            eprintln!("failed to send a message: {}", err);
            process::exit(1);
        });
    });
    let received = match rx.recv() {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("failed to send a message: {}", err);
            process::exit(1);
        }
    };
    println!("got: {}", received);
}
