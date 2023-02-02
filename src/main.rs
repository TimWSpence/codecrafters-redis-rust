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

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    while let Some(req) = Request::decode(&mut reader) {
        handle_request(req, stream.try_clone().unwrap());
    }
}

fn handle_request(req: Request, mut stream: TcpStream) {
    match req {
        Request::Ping { data } => match data {
            Some(d) => stream.write_all(bulk_string(d).as_bytes()).unwrap(),
            _ => stream.write_all(simple_string("PONG").as_bytes()).unwrap(),
        },
    };
}

fn simple_string(s: &str) -> String {
    format!("+{}\r\n", s)
}

fn bulk_string(s: String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}
