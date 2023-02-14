use std::io::Write;
use std::net::TcpStream;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::model::*;

pub fn handle_request(req: Request, store: Store, stream: TcpStream) {
    match req {
        Request::Ping { data } => handle_ping(data, stream),
        Request::Echo { data } => handle_echo(data, stream),
        Request::Get { key } => handle_get(key, store, stream),
        Request::Set { key, value, expiry } => handle_set(key, value, expiry, store, stream),
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
        Some(e) => match &e.expiry {
            Some(t) => {
                let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                if current > *t {
                    stream.write_all(NIL).unwrap()
                } else {
                    stream.write_all(bulk_string(&e.value).as_bytes()).unwrap()
                }
            }
            _ => stream.write_all(bulk_string(&e.value).as_bytes()).unwrap(),
        },

        _ => stream.write_all(NIL).unwrap(),
    }
}

fn handle_set(
    key: String,
    value: String,
    expiry: Option<u64>,
    store: Store,
    mut stream: TcpStream,
) {
    let mut store = store.lock().unwrap();
    let expiry = match expiry {
        Some(t) => {
            let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            Some(current + Duration::from_millis(t))
        }
        _ => None,
    };
    let entry = StoreEntry { value, expiry };
    store.insert(key, entry);
    stream.write_all(simple_string("OK").as_bytes()).unwrap();
}

fn simple_string(s: &str) -> String {
    format!("+{}\r\n", s)
}

fn bulk_string(s: &String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}

const NIL: &'static [u8] = "$-1\r\n".as_bytes();
