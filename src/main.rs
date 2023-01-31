// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;
mod model;
use model::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                _stream.write(&Response::Pong.encode()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
