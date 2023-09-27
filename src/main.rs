use shrew::{ HttpStatus, stream::Stream };
use shrew::response::Response;

use std::net::{TcpListener, TcpStream};


fn handle_client(connection: &mut TcpStream) -> std::io::Result<()> {
    let mut stream = Stream(connection);

    // Read HTTP request, and parse it.
    let request = match stream.get_next_request() {
        Ok(r) => r,
        Err(error) => {
            match error {
                _ => todo!("Handle request error")
            }
        }
    };
    let mut response: Response = Response::new(&mut stream);

    response.send_status(HttpStatus::TemporaryRedirect)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("[::1]:8080")?;

    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }

    Ok(())
}