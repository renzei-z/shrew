use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use crate::request::Request;

pub struct Stream<'s>(&'s mut TcpStream);

impl<'s> Stream<'s> {
    pub fn from_stream(stream: &mut TcpStream) -> Stream {
        Stream(stream)
    }

    pub(crate) fn write(&mut self, bytes: &str) -> std::io::Result<()> {
        self.0.write( bytes.as_bytes())?;
        self.0.flush()
    }

    pub(crate) fn close(&mut self) -> std::io::Result<()> {
        self.0.flush()?;
        self.0.shutdown(std::net::Shutdown::Both)
    }

    // TODO: We will change this so that we can handle
    // more than one request at a time, but for now,
    // just get the next request available to the stream.
    pub fn get_request(self) -> Option<Request> {        
        let mut buf = String::new();
        let mut request = String::new();
        let mut reader = BufReader::new(self.0);

        loop {
            let s = reader.read_line(&mut buf).unwrap();

            request.push_str(&buf);

            buf = "".to_string();

            // Empty line
            if s < 3 {
                break;
            }
        }

        buf = "".to_string();
        // Now we check for Content-Length
        if let Some(l) = request.lines().next() {
            // We don't want to read the body if it's a GET request.
            // For now, we only read the body if the Content-Length header was
            // sent, however in the future we will check for a body anyway
            // if this header is missing.
            if !l.contains("GET")  {
                // As much as it pains me to have this much nesting, Rust doesn't
                // yet support if let and regular if in the same expression yet.
                if let Some(length_index) = request.find("Content-Length: ") {
                    let content_length = &request[length_index..].lines().next().unwrap().split(": ").skip(1).next().unwrap().parse::<usize>();
                    match content_length {
                        Ok(l) => {
                            let mut buffer = vec![0; *l];
                            // Handle error later :)
                            reader.read_exact(&mut buffer).unwrap();
                            request.push_str(std::str::from_utf8(&buffer).unwrap());
                        },
                        // Do nothing if corrupted Content-Length. We'll ignore the body for now.
                        Err(_) => {}
                    }
                }
            }
        }

        request = request.trim_end_matches("\0").to_string();

        println!("{}", request);

        match request.len() {
            0 => None,
            _ => match Request::try_from(request) {
                Ok(req) => Some(req),
                Err(e) => {
                    eprintln!("ERROR: Error parsing request: {e}");
                    return None
                }
            }
        }
    }
} 