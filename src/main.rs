use shrew::HttpStatus;
use shrew::request::{Request, RequestError};
use shrew::response::Response;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};

struct Stream<'tcp>(&'tcp mut TcpStream);

impl Stream<'_> {
    fn send_response(&mut self, response: Response) -> std::io::Result<()> {
        self.write(&response.as_http_response())?;
        self.close_connection()
    }

    fn write(&mut self, bytes: &str) -> std::io::Result<()> {
        write!(self.0, "{}", bytes)?;
        self.flush()
    }
    
    fn send_and_close(&mut self, bytes: &str) -> std::io::Result<()> {
        self.write(bytes)?;
        self.close_connection()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }

    fn close_connection(&mut self) -> std::io::Result<()> {
        self.0.shutdown(Shutdown::Both)
    }
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut stream = Stream(stream);

    let mut buffer = [0; 1024];
    let mut message = String::new();
    loop {
        // Read into the buffer and get the byte count read.
        let byte_count = stream.0.read(&mut buffer)?;

        // If no bytes read, then we're finished.
        // TODO: This could also mean that the request has blocked halfway through,
        // in which case we should give some time and then throw a 408.
        if byte_count == 0 {
            break;
        }

        // TODO: Check on how to parse encoding from a header.. for a message
        // we're receiving which contains the header??
        message.push_str(std::str::from_utf8(&buffer).unwrap());
    
        // TODO: Handle requests with bodies (e.g. POST requests)
        if message.contains("\r\n\r\n") {
            break;
        }
    }

    let mut response = Response::new();

    let request = match Request::from_string(message) {
        Ok(request) => request,
        Err(error) => {
            match error {
                RequestError::UnknownMethod(_method) => {
                    response.set_status(HttpStatus::NotImplemented);
                    stream.send_response(response)?;
                    return Ok(())
                },
                #[allow(unreachable_patterns)]
                _ => todo!("Handle parsing errors, and server errors (e.g. 505, 501, 411 etc)")
            }
            
        }
    };
    
    request.pretty_print();

    stream.send_response(response)?;
    stream.flush()?;
    stream.close_connection()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("[::1]:8080")?;

    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }

    Ok(())
}