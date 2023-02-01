use std::io::{BufRead, Read};
use std::{io::BufReader, net::TcpStream};

pub enum Request<'a> {
    Ping { data: Option<&'a str> },
}

impl<'a> Request<'a> {
    pub fn decode(reader: &mut BufReader<TcpStream>) -> Option<Request> {
        fn read_bulk_string<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
        ) -> Option<&'a str> {
            buf.clear();
            let len = {
                let n = reader.read_line(buf).ok()?;
                if n == 0 {
                    None
                } else {
                    let s = buf.trim_end();
                    Request::decode_usize(&s[1..])
                }
            }?;
            buf.clear();
            let s = {
                let n = reader.read_line(buf).ok()?;
                if n != len {
                    None
                } else {
                    Some(buf.trim_end())
                }
            }?;
            Some(s)
        };

        fn parse_ping<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
            len: usize,
        ) -> Option<Request<'a>> {
            if len == 1 {
                Some(Request::Ping { data: None })
            } else {
                buf.clear();
                let data = read_bulk_string(reader, buf)?;
                Some(Request::Ping { data: Some(data) })
            }
        }

        let mut buf = String::new();
        let len = {
            let n = reader.read_line(&mut buf).ok()?;
            if n == 0 {
                None
            } else {
                let s = buf.trim_end();
                if s.starts_with("*") {
                    Request::decode_usize(&s[1..])
                } else {
                    None
                }
            }
        }?;
        let cmd = {
            if len == 0 {
                None
            } else {
                read_bulk_string(reader, &mut buf)
            }
        }?;
        match cmd {
            "PING" => parse_ping(reader, &mut buf, len),
            _ => None,
        }
    }

    fn decode_usize(data: &str) -> Option<usize> {
        data.parse::<usize>().ok()
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
