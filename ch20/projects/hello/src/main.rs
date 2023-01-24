use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
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
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(err) = stream.read(&mut buffer) {
        eprintln!("error occured: {}", err);
        process::exit(1);
    }

    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        let mut contents = String::new();
        let mut file = match File::open("hello.html") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("error occured: {}", err);
                process::exit(1);
            }
        };

        if let Err(err) = file.read_to_string(&mut contents) {
            eprintln!("error occured: {}", err);
            process::exit(1);
        }
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        if let Err(err) = stream
            .write(response.as_bytes())
            .and_then(|_| stream.flush())
        {
            eprintln!("error occured: {}", err);
            process::exit(1);
        }
    } else {
        let mut contents = String::new();
        let mut file = match File::open("404.html") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("error occured: {}", err);
                process::exit(1);
            }
        };

        if let Err(err) = file.read_to_string(&mut contents) {
            eprintln!("error occured: {}", err);
            process::exit(1);
        }
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);

        if let Err(err) = stream
            .write(response.as_bytes())
            .and_then(|_| stream.flush())
        {
            eprintln!("error occured: {}", err);
            process::exit(1);
        }
    }
}
