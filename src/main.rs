use std::collections::HashMap;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
mod model;
use model::*;
mod handlers;
use handlers::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let store = store.clone();
                thread::spawn(|| {
                    handle_connection(store, _stream);
                });
                
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(store: Store, stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    while let Some(req) = Request::decode(&mut reader) {
        let store = store.clone();
        handle_request(req, store, stream.try_clone().unwrap());
    }
}
