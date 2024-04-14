// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf);
    let _ = stream.write_all(String::from("+PONG\r\n").as_bytes());
}
