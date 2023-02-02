use std::io::BufRead;
use std::{io::BufReader, net::TcpStream};

#[derive(Debug)]
pub enum Request {
    Ping { data: Option<String> },
}

impl Request {
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
                    decode_usize(&s[1..])
                }
            }?;
            buf.clear();
            let s = {
                let n = reader.read_line(buf).ok()?;
                if n != (len + 2) {
                    None
                } else {
                    Some(buf.trim_end())
                }
            }?;
            Some(s)
        }

        fn parse_ping<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
            len: usize,
        ) -> Option<Request> {
            //TODO allocate buffer here instead?
            if len == 1 {
                Some(Request::Ping { data: None })
            } else {
                buf.clear();
                let data = read_bulk_string(reader, buf)?;
                Some(Request::Ping {
                    data: Some(data.to_string()),
                })
            }
        }

        fn decode_usize(data: &str) -> Option<usize> {
            data.parse::<usize>().ok()
        }

        let mut buf = String::new();
        let len = {
            let n = reader.read_line(&mut buf).ok()?;
            if n == 0 {
                None
            } else {
                let s = buf.trim_end();
                if s.starts_with("*") {
                    decode_usize(&s[1..])
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
}
