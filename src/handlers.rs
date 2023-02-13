use std::io::Write;
use std::net::TcpStream;

use crate::model::*;

pub fn handle_request(req: Request, stream: TcpStream) {
    match req {
        Request::Ping { data } => handle_ping(data, stream),
        Request::Echo { data } => handle_echo(data, stream),
    };
}

fn handle_ping(data: Option<String>, mut stream: TcpStream) {
    match data {
        Some(d) => stream.write_all(bulk_string(d).as_bytes()).unwrap(),
        _ => stream.write_all(simple_string("PONG").as_bytes()).unwrap(),
    }
}

fn handle_echo(data: String, mut stream: TcpStream) {
    stream.write_all(bulk_string(data).as_bytes()).unwrap()
}

fn simple_string(s: &str) -> String {
    format!("+{}\r\n", s)
}

fn bulk_string(s: String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}
