pub enum Response {
    Pong,
}

impl RESPEncoder for Response {
    fn encode(&self) -> Vec<u8> {
        match self {
            Response::Pong => "+PONG\r\n".as_bytes().to_vec(),
        }
    }
}

pub trait RESPEncoder {
    fn encode(&self) -> Vec<u8>;
}
