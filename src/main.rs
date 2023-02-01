use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
mod model;
use model::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    while let Some(req) = Request::decode(&mut reader) {
        match req {
            Request::Ping { data: _ } => {
                stream.write_all(&Response::Pong.encode()).unwrap();
            }
        }
    }
}
