use std::io::BufRead;
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
    let mut buf = String::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            //EOF
            break;
        };
        if let Some(req) = Request::decode(buf.trim_end()) {
            match req {
                Request::Ping => {
                    stream.write_all(&Response::Pong.encode()).unwrap();
                }
            }
        } else {
            println!("Failed to decode {}", buf)
        }
    }
}
