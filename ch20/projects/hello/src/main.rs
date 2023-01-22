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
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    if let Err(err) = stream
        .read(&mut buffer)
        .and_then(|_| stream.write(response.as_bytes()))
        .and_then(|_| stream.flush())
    {
        eprintln!("error occured: {}", err);
        process::exit(1);
    }
}
