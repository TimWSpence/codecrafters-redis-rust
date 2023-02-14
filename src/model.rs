use std::collections::HashMap;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io::BufReader, net::TcpStream};

pub type Store = Arc<Mutex<HashMap<String, StoreEntry>>>;

pub struct StoreEntry {
    pub value: String,
    pub expiry: Option<Duration>,
}

#[derive(Debug)]
pub enum Request {
    Ping {
        data: Option<String>,
    },
    Echo {
        data: String,
    },
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        expiry: Option<u64>,
    },
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

        fn parse_echo<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
            len: usize,
        ) -> Option<Request> {
            if len == 1 {
                None
            } else {
                buf.clear();
                let data = read_bulk_string(reader, buf)?;
                Some(Request::Echo {
                    data: data.to_string(),
                })
            }
        }

        fn parse_get<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
            len: usize,
        ) -> Option<Request> {
            if len == 1 {
                None
            } else {
                buf.clear();
                let key = read_bulk_string(reader, buf)?;
                Some(Request::Get {
                    key: key.to_string(),
                })
            }
        }

        fn parse_set<'a>(
            reader: &mut BufReader<TcpStream>,
            buf: &'a mut String,
            len: usize,
        ) -> Option<Request> {
            if len != 3 && len != 5 {
                None
            } else {
                let key = {
                    buf.clear();
                    let s = read_bulk_string(reader, buf)?;
                    s.to_string()
                };
                let value = {
                    buf.clear();
                    let s = read_bulk_string(reader, buf)?;
                    s.to_string()
                };
                if len == 3 {
                    Some(Request::Set {
                        key: key.to_string(),
                        value: value.to_string(),
                        expiry: None,
                    })
                } else {
                    //TODO parse expiry
                    {
                        buf.clear();
                        let s = read_bulk_string(reader, buf)?;
                        assert!(s.to_uppercase() == "PX");
                    };
                    let expiry = {
                        buf.clear();
                        let s = read_bulk_string(reader, buf)?;
                        s.to_string()
                    };
                    let expiry = expiry.parse::<u64>().unwrap();
                    Some(Request::Set {
                        key: key.to_string(),
                        value: value.to_string(),
                        expiry: Some(expiry),
                    })
                }
            }
        }

        fn decode_usize(data: &str) -> Option<usize> {
            data.parse::<usize>().ok()
        }

        fn read_array_length(reader: &mut BufReader<TcpStream>, buf: &mut String) -> Option<usize> {
            buf.clear();
            let n = reader.read_line(buf).ok()?;
            if n == 0 {
                None
            } else {
                let s = buf.trim_end();
                if s.starts_with("*") {
                    s[1..].parse::<usize>().ok()
                } else {
                    None
                }
            }
        }

        let mut buf = String::new();
        let len = read_array_length(reader, &mut buf)?;
        let cmd = {
            if len == 0 {
                None
            } else {
                read_bulk_string(reader, &mut buf)
            }
        }?;
        match cmd.to_uppercase().as_str() {
            "PING" => parse_ping(reader, &mut buf, len),
            "ECHO" => parse_echo(reader, &mut buf, len),
            "GET" => parse_get(reader, &mut buf, len),
            "SET" => parse_set(reader, &mut buf, len),
            _ => None,
        }
    }
}
