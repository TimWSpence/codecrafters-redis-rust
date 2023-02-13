use std::io::Write;
use std::net::TcpStream;

use crate::model::*;

pub fn handle_request(req: Request, store: Store, stream: TcpStream) {
    match req {
        Request::Ping { data } => handle_ping(data, stream),
        Request::Echo { data } => handle_echo(data, stream),
        Request::Get { key } => handle_get(key, store, stream),
        Request::Set { key, value } => handle_set(key, value, store, stream),
    };
}

fn handle_ping(data: Option<String>, mut stream: TcpStream) {
    match data {
        Some(d) => stream.write_all(bulk_string(&d).as_bytes()).unwrap(),
        _ => stream.write_all(simple_string("PONG").as_bytes()).unwrap(),
    }
}

fn handle_echo(data: String, mut stream: TcpStream) {
    stream.write_all(bulk_string(&data).as_bytes()).unwrap()
}

fn handle_get(key: String, store: Store, mut stream: TcpStream) {
    let store = store.lock().unwrap();
    match store.get(&key) {
        Some(v) => stream.write_all(bulk_string(v).as_bytes()).unwrap(),
        _ => stream.write_all(nil()).unwrap(),
    }
}

fn handle_set(key: String, value: String, store: Store, mut stream: TcpStream) {
    let mut store = store.lock().unwrap();
    store.insert(key, value);
    stream.write_all(simple_string("OK").as_bytes()).unwrap();
}

fn simple_string(s: &str) -> String {
    format!("+{}\r\n", s)
}

fn bulk_string(s: &String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}

fn nil() -> &'static [u8] {
    "$-1\r\n".as_bytes()
}
