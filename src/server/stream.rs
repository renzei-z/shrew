use std::io::prelude::*;
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
        let mut buf: [u8; 4098] = [0; 4098];
        let mut request = String::new();

        loop {
            let s = self.0.read(&mut buf).unwrap();

            if s == 0 {
                break;
            }

            request.push_str(std::str::from_utf8(&buf).unwrap());

            // TODO: We ignore the body for now, since we only
            // handle GET requests for now.
            if request.contains("\r\n\r\n") {
                break;
            }
        }

        request = request.trim_end_matches("\0").to_string();

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