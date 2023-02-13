use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
mod model;
use model::*;
mod handlers;
use handlers::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(|| {
                    handle_connection(_stream);
                });
                ()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    while let Some(req) = Request::decode(&mut reader) {
        handle_request(req, stream.try_clone().unwrap());
    }
}
