pub enum Request {
    Ping,
}

impl Request {
    pub fn decode(data: &str) -> Option<Request> {
        if (data == ("+PING")) {
            Some(Request::Ping)
        } else {
            None
        }
    }
}

pub enum Response {
    Pong,
}

impl Response {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Response::Pong => "+PONG\r\n".as_bytes().to_vec(),
        }
    }
}
