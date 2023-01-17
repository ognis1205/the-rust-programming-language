use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for msg in msgs {
            println!("sent: {}", msg);
            tx.send(msg).unwrap_or_else(|err| {
                eprintln!("failed to send a message: {}", err);
                process::exit(1);
            });
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("got: {}", received);
    }
}
