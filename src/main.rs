// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::thread;

mod evaluator;
mod output;
mod parser;
mod types;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("");

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
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(_n) => {
                let response = handle_connection(&buf);
                let _ = stream.write_all(response.as_bytes());
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}

fn handle_connection(input: &[u8]) -> String {
    let command = from_utf8(input).unwrap().trim_matches(char::from(0));
    let command = parser::Parser(command.chars())
        .map(TryInto::try_into)
        .collect();
    evaluator::eval(command)
}
