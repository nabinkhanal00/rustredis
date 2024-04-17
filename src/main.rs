// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .expect("Failed to bind listener to the provided address.");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handle each stream in a different thread
                thread::spawn(move || {
                    let _ = handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        let request = String::from_utf8(buf[..n].to_vec())?;
        let request = request.trim();
        let mut request = request.split_whitespace();

        let command = request.next().ok_or("Command is not provided.")?;
        let command = command.to_lowercase();
        let command = command.as_str();
        match command {
            "ping" => {
                let _ = stream.write_all("+PONG\r\n".as_bytes());
            }
            "echo" => {
                let message = request.next().unwrap_or("");
                let message = format!("${}{}\r\n", message.len(), message);
                let _ = stream.write_all(message.as_bytes());
            }
            _ => {}
        }
    }
}
