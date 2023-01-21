use std::net::TcpListener;
use std::process;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("error occured: {}", err);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("error occured: {}", err);
                process::exit(1);
            }
        };
        println!("connection established");
    }
}
