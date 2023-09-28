use crate::request::Request;

use std::io::prelude::*;
use std::net::{ TcpStream, Shutdown };

pub struct Stream<'tcp>(pub &'tcp mut TcpStream);

pub enum StreamError {

}

impl Stream<'_> {
    pub fn get_next_request(&mut self) -> std::io::Result<Request> {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut message = String::new();

        loop {
            let byte_count = match self.0.read(&mut buffer) {
                Ok(count) => count,
                Err(_e) => {
                    panic!("Error reading request into buffer. Quitting.");
                }
            };

            if byte_count == 0 { break; }

            message.push_str(std::str::from_utf8(&buffer).unwrap());

            if message.contains("\r\n\r\n") { break; }
        }

        println!("{}", message);

        Ok(Request::new())
    }

    pub fn write(&mut self, message: &str) -> std::io::Result<()> {
        write!(self.0, "{}", message)?;
        self.0.flush()
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        self.0.shutdown(Shutdown::Both)
    }
}
